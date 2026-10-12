use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: usize,
    pub name: String,
    pub x: f64,
    pub y: f64,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub from: usize,
    pub to: usize,
    pub km: f64,
    pub speed_kph: f64,
    pub mode: String,
    pub max_tonnes: f64,
    pub max_height_m: f64,
    pub grade_pct: f64,
    pub weather: f64,
    pub headway_minutes: Option<f64>,
    pub transfer_minutes: Option<f64>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Graph {
    pub revision: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Closure {
    pub edge_id: String,
    pub start: f64,
    pub end: f64,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Request {
    pub origin: usize,
    pub destination: usize,
    pub departure_minutes: f64,
    pub weight_tonnes: f64,
    pub height_m: f64,
    pub max_grade_pct: f64,
    pub weather_severity: f64,
    pub allow_ferry: bool,
    pub closures: Vec<Closure>,
}
#[derive(Clone, Serialize)]
pub struct Leg {
    pub edge_id: String,
    pub from: usize,
    pub to: usize,
    pub mode: String,
    pub km: f64,
    pub entered_at: f64,
    pub exited_at: f64,
    pub drive_minutes: f64,
    pub closure_wait_minutes: f64,
    pub schedule_wait_minutes: f64,
    pub transfer_minutes: f64,
}
#[derive(Serialize)]
pub struct Result {
    pub graph_revision: String,
    pub arrival_minutes: f64,
    pub duration_minutes: f64,
    pub distance_km: f64,
    pub expanded_states: usize,
    pub legs: Vec<Leg>,
    pub exclusions: Vec<Exclusion>,
}
#[derive(Serialize)]
pub struct Exclusion {
    pub edge_id: String,
    pub reason: String,
}

// Node × incoming transport mode is the search state. Retaining mode is essential:
// the earliest arrival at a terminal need not dominate an arrival in another mode.
#[derive(Clone, Copy)]
struct Label {
    time: f64,
    state: usize,
}
impl PartialEq for Label {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.state == other.state
    }
}
impl Eq for Label {}
impl Ord for Label {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .time
            .total_cmp(&self.time)
            .then_with(|| other.state.cmp(&self.state))
    }
}
impl PartialOrd for Label {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn mode_id(mode: &str) -> usize {
    if mode == "ferry" {
        1
    } else {
        0
    }
}

impl Graph {
    pub fn validate(&self) -> std::result::Result<(), String> {
        let mut ids = HashSet::new();
        for (i, node) in self.nodes.iter().enumerate() {
            if node.id != i {
                return Err("Node IDs must be contiguous".into());
            }
        }
        for e in &self.edges {
            if e.from >= self.nodes.len()
                || e.to >= self.nodes.len()
                || !ids.insert(&e.id)
                || ![e.km, e.speed_kph, e.max_tonnes, e.max_height_m]
                    .iter()
                    .all(|x| x.is_finite() && *x > 0.0)
                || !e.grade_pct.is_finite()
                || e.grade_pct < 0.0
                || !e.weather.is_finite()
                || !(0.0..=1.0).contains(&e.weather)
                || !["ground", "ferry"].contains(&e.mode.as_str())
                || (e.mode == "ferry"
                    && !e.headway_minutes.is_some_and(|x| x.is_finite() && x > 0.0))
                || e.transfer_minutes
                    .is_some_and(|x| !x.is_finite() || x < 0.0)
            {
                return Err(format!("Invalid edge {}", e.id));
            }
        }
        Ok(())
    }
}
fn exclusion(e: &Edge, r: &Request) -> Option<String> {
    if r.weight_tonnes > e.max_tonnes {
        Some(format!("Weight exceeds {} t limit", e.max_tonnes))
    } else if r.height_m > e.max_height_m {
        Some(format!("Height exceeds {} m clearance", e.max_height_m))
    } else if r.max_grade_pct < e.grade_pct {
        Some(format!("Grade exceeds vehicle {}% limit", r.max_grade_pct))
    } else if e.mode == "ferry" && !r.allow_ferry {
        Some("Ferry transport disabled".into())
    } else {
        None
    }
}
fn traverse(e: &Edge, from: usize, incoming_mode: usize, arrival: f64, r: &Request) -> Leg {
    let transfer = if incoming_mode != mode_id(&e.mode) {
        e.transfer_minutes.unwrap_or(6.0)
    } else {
        0.0
    };
    let ready = arrival + transfer;
    let drive = 60.0 * e.km / e.speed_kph * (1.0 + r.weather_severity * e.weather * 2.0);
    let mut depart = ready;
    let mut closure_wait = 0.0;
    let mut schedule_wait = 0.0;
    // Re-evaluate timetable after every closure delay. Half-open closures block
    // traversal if any of [depart, arrival) overlaps [start, end).
    loop {
        if let Some(headway) = e.headway_minutes {
            let scheduled = (depart / headway).ceil() * headway;
            schedule_wait += scheduled - depart;
            depart = scheduled;
        }
        let blocked_until = r
            .closures
            .iter()
            .filter(|c| c.edge_id == e.id && depart < c.end && depart + drive > c.start)
            .map(|c| c.end)
            .fold(depart, f64::max);
        if blocked_until <= depart {
            break;
        }
        closure_wait += blocked_until - depart;
        depart = blocked_until;
    }
    Leg {
        edge_id: e.id.clone(),
        from,
        to: if from == e.from { e.to } else { e.from },
        mode: e.mode.clone(),
        km: e.km,
        entered_at: depart,
        exited_at: depart + drive,
        drive_minutes: drive,
        closure_wait_minutes: closure_wait,
        schedule_wait_minutes: schedule_wait,
        transfer_minutes: transfer,
    }
}
pub fn solve(g: &Graph, r: &Request) -> std::result::Result<Result, String> {
    if r.origin >= g.nodes.len() || r.destination >= g.nodes.len() {
        return Err("Unknown origin or destination".into());
    }
    if ![
        r.departure_minutes,
        r.weight_tonnes,
        r.height_m,
        r.max_grade_pct,
        r.weather_severity,
    ]
    .iter()
    .all(|x| x.is_finite())
        || !(0.0..=1440.0).contains(&r.departure_minutes)
        || r.weight_tonnes <= 0.0
        || r.height_m <= 0.0
        || r.max_grade_pct < 0.0
        || !(0.0..=1.0).contains(&r.weather_severity)
        || r.closures.len() > 256
    {
        return Err("Invalid vehicle, departure, weather, or closure count".into());
    }
    for c in &r.closures {
        if !c.start.is_finite()
            || !c.end.is_finite()
            || c.start < 0.0
            || c.end <= c.start
            || c.end > 10080.0
            || !g.edges.iter().any(|e| e.id == c.edge_id)
        {
            return Err("Invalid closure window or edge ID".into());
        }
    }
    let exclusions: Vec<_> = g
        .edges
        .iter()
        .filter_map(|e| {
            exclusion(e, r).map(|reason| Exclusion {
                edge_id: e.id.clone(),
                reason,
            })
        })
        .collect();
    let mut adjacency = vec![Vec::new(); g.nodes.len()];
    for (i, e) in g.edges.iter().enumerate() {
        if exclusion(e, r).is_none() {
            adjacency[e.from].push(i);
            adjacency[e.to].push(i);
        }
    }
    let mut dist = vec![f64::INFINITY; g.nodes.len() * 2];
    let mut previous: Vec<Option<(usize, Leg)>> = vec![None; dist.len()];
    let start = r.origin * 2;
    dist[start] = r.departure_minutes;
    let mut queue = BinaryHeap::from([Label {
        time: r.departure_minutes,
        state: start,
    }]);
    let mut expanded = 0;
    let mut goal = None;
    while let Some(Label { time, state }) = queue.pop() {
        if time > dist[state] {
            continue;
        }
        expanded += 1;
        if state / 2 == r.destination {
            goal = Some(state);
            break;
        }
        for &index in &adjacency[state / 2] {
            let edge = &g.edges[index];
            let leg = traverse(edge, state / 2, state % 2, time, r);
            let next = leg.to * 2 + mode_id(&edge.mode);
            if leg.exited_at < dist[next] {
                dist[next] = leg.exited_at;
                queue.push(Label {
                    time: leg.exited_at,
                    state: next,
                });
                previous[next] = Some((state, leg));
            }
        }
    }
    let mut state = goal.ok_or("No feasible route for this vehicle")?;
    let arrival = dist[state];
    let mut legs = Vec::new();
    while state != start {
        let (parent, leg) = previous[state].clone().expect("settled predecessor");
        legs.push(leg);
        state = parent;
    }
    legs.reverse();
    Ok(Result {
        graph_revision: g.revision.clone(),
        arrival_minutes: arrival,
        duration_minutes: arrival - r.departure_minutes,
        distance_km: legs.iter().map(|l| l.km).sum(),
        expanded_states: expanded,
        legs,
        exclusions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn graph() -> Graph {
        serde_json::from_str(include_str!("../data/network.json")).unwrap()
    }
    fn request() -> Request {
        Request {
            origin: 0,
            destination: 4,
            departure_minutes: 480.0,
            weight_tonnes: 18.0,
            height_m: 3.5,
            max_grade_pct: 10.0,
            weather_severity: 0.0,
            allow_ferry: false,
            closures: vec![],
        }
    }
    #[test]
    fn baseline_uses_bridge() {
        let g = graph();
        g.validate().unwrap();
        let route = solve(&g, &request()).unwrap();
        assert!(route.legs.iter().any(|l| l.edge_id == "river-bridge"));
        assert_eq!(route.distance_km, 32.0);
    }
    #[test]
    fn heavy_truck_avoids_bridge() {
        let mut r = request();
        r.weight_tonnes = 35.0;
        let result = solve(&graph(), &r).unwrap();
        assert_eq!(result.distance_km, 43.0);
    }
    #[test]
    fn closure_changes_path() {
        let mut r = request();
        r.closures.push(Closure {
            edge_id: "river-bridge".into(),
            start: 480.0,
            end: 720.0,
        });
        let route = solve(&graph(), &r).unwrap();
        assert!(!route.legs.iter().any(|l| l.edge_id == "river-bridge"));
    }
    #[test]
    fn ferry_enables_heavy_vehicle() {
        let mut r = request();
        r.weight_tonnes = 35.0;
        r.max_grade_pct = 4.0;
        r.allow_ferry = true;
        let result = solve(&graph(), &r).unwrap();
        assert!(result.legs.iter().any(|l| l.mode == "ferry"));
    }
    #[test]
    fn impossible_vehicle() {
        let mut r = request();
        r.weight_tonnes = 90.0;
        assert!(solve(&graph(), &r).is_err());
    }
    #[test]
    fn zero_length_route() {
        let mut r = request();
        r.destination = r.origin;
        let result = solve(&graph(), &r).unwrap();
        assert_eq!(result.duration_minutes, 0.0);
        assert!(result.legs.is_empty());
    }
    #[test]
    fn closures_block_whole_traversal_and_recheck_timetable() {
        let g = graph();
        let e = g.edges.iter().find(|e| e.mode == "ferry").unwrap();
        let mut r = request();
        r.closures = vec![Closure {
            edge_id: e.id.clone(),
            start: 511.0,
            end: 541.0,
        }];
        let leg = traverse(e, e.from, 0, 500.0, &r);
        assert_eq!(leg.entered_at, 570.0);
        assert_eq!(leg.transfer_minutes, 6.0);
    }
    #[test]
    fn closure_end_is_open() {
        let g = graph();
        let e = &g.edges[0];
        let mut r = request();
        r.closures = vec![Closure {
            edge_id: e.id.clone(),
            start: 480.0,
            end: 500.0,
        }];
        assert_eq!(traverse(e, e.from, 0, 500.0, &r).closure_wait_minutes, 0.0);
    }
    #[test]
    fn invalid_closure_rejected() {
        let mut r = request();
        r.closures = vec![Closure {
            edge_id: "missing".into(),
            start: 0.0,
            end: 500.0,
        }];
        assert!(solve(&graph(), &r).is_err());
    }
    #[test]
    fn earlier_departure_never_arrives_later() {
        let g = graph();
        let mut r = request();
        r.allow_ferry = true;
        r.weight_tonnes = 35.0;
        r.max_grade_pct = 4.0;
        let mut last = 0.0;
        for t in 450..550 {
            r.departure_minutes = t as f64;
            let arrival = solve(&g, &r).unwrap().arrival_minutes;
            assert!(arrival >= last);
            last = arrival;
        }
    }
}
