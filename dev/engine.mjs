// Runnable reference implementation for machines without a Rust toolchain.
// API and cost semantics mirror src/routing.rs. This is not the Rust binary.
export function exclusion(e, r) {
  if (r.weight_tonnes > e.max_tonnes) return `Weight exceeds ${e.max_tonnes} t limit`;
  if (r.height_m > e.max_height_m) return `Height exceeds ${e.max_height_m} m clearance`;
  if (r.max_grade_pct < e.grade_pct) return `Grade exceeds vehicle ${r.max_grade_pct}% limit`;
  if (e.mode === 'ferry' && !r.allow_ferry) return 'Ferry transport disabled';
  return null;
}
export function traverse(e, from, incomingMode, arrival, r) {
  const mode = e.mode === 'ferry' ? 1 : 0;
  const transfer = incomingMode !== mode ? (e.transfer_minutes ?? 6) : 0;
  const drive = 60 * e.km / e.speed_kph * (1 + r.weather_severity * e.weather * 2);
  let depart = arrival + transfer, closureWait = 0, scheduleWait = 0;
  for (;;) {
    if (e.headway_minutes) {
      const next = Math.ceil(depart / e.headway_minutes) * e.headway_minutes;
      scheduleWait += next - depart; depart = next;
    }
    const end = Math.max(depart, ...r.closures.filter(c => c.edge_id === e.id && depart < c.end && depart + drive > c.start).map(c => c.end));
    if (end <= depart) break;
    closureWait += end - depart; depart = end;
  }
  return {edge_id:e.id, from, to:from === e.from ? e.to : e.from, mode:e.mode, km:e.km,
    entered_at:depart, exited_at:depart + drive, drive_minutes:drive,
    closure_wait_minutes:closureWait, schedule_wait_minutes:scheduleWait, transfer_minutes:transfer};
}
export function solve(g, r) {
  if (!Number.isInteger(r.origin) || !Number.isInteger(r.destination) || !g.nodes[r.origin] || !g.nodes[r.destination]) throw Error('Unknown origin or destination');
  if (![r.departure_minutes,r.weight_tonnes,r.height_m,r.max_grade_pct,r.weather_severity].every(Number.isFinite)
    || r.departure_minutes < 0 || r.departure_minutes > 1440 || r.weight_tonnes <= 0 || r.height_m <= 0
    || r.max_grade_pct < 0 || r.weather_severity < 0 || r.weather_severity > 1 || typeof r.allow_ferry !== 'boolean'
    || !Array.isArray(r.closures) || r.closures.length > 256) throw Error('Invalid vehicle, departure, weather, or closure count');
  for (const c of r.closures) if (!Number.isFinite(c.start) || !Number.isFinite(c.end) || c.start < 0 || c.end <= c.start || c.end > 10080 || !g.edges.some(e => e.id === c.edge_id)) throw Error('Invalid closure window or edge ID');
  const exclusions = g.edges.map(e => ({edge_id:e.id, reason:exclusion(e,r)})).filter(e => e.reason);
  const adjacency = g.nodes.map(() => []);
  g.edges.forEach(e => { if (!exclusion(e,r)) { adjacency[e.from].push(e);adjacency[e.to].push(e); } });
  const dist = Array(g.nodes.length * 2).fill(Infinity), prev = Array(dist.length).fill(null);
  const start = r.origin * 2;dist[start] = r.departure_minutes;
  // Tiny reference graph uses a sorted frontier; Rust uses BinaryHeap.
  const queue = [{state:start, time:r.departure_minutes}];let expanded = 0, goal;
  while (queue.length) {
    queue.sort((a,b) => b.time-a.time || b.state-a.state);
    const {state,time} = queue.pop();if (time > dist[state]) continue;
    expanded++;if (Math.floor(state/2) === r.destination) {goal=state;break;}
    for (const edge of adjacency[Math.floor(state/2)]) {
      const leg = traverse(edge, Math.floor(state/2), state%2, time, r);
      const next = leg.to*2 + (edge.mode === 'ferry' ? 1 : 0);
      if (leg.exited_at < dist[next]) {dist[next]=leg.exited_at;prev[next]={state,leg};queue.push({state:next,time:leg.exited_at});}
    }
  }
  if (goal === undefined) throw Error('No feasible route for this vehicle');
  const arrival = dist[goal], legs=[];
  while (goal !== start) {legs.push(prev[goal].leg);goal=prev[goal].state;}
  legs.reverse();
  return {graph_revision:g.revision,arrival_minutes:arrival,duration_minutes:arrival-r.departure_minutes,
    distance_km:legs.reduce((s,l)=>s+l.km,0),expanded_states:expanded,legs,exclusions};
}
