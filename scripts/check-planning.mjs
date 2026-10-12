import {readFileSync} from 'node:fs';
import assert from 'node:assert/strict';

const root = new URL('../', import.meta.url);
const {issues} = JSON.parse(readFileSync(new URL('planning/backlog.json', root), 'utf8'));
const project = JSON.parse(readFileSync(new URL('planning/project.json', root), 'utf8'));
const ids = new Set(issues.map(issue => issue.id));
const milestones = new Set(project.milestones.map(m => m.id));
const labels = new Set(project.labels.map(label => label.name));
assert.equal(ids.size, issues.length, 'Planning IDs must be unique');
const byId = new Map(issues.map(issue => [issue.id, issue]));
for (const issue of issues) {
  assert.match(issue.id, /^SUS-\d{3}$/);
  assert.ok(issue.title && issue.description, `${issue.id}: missing description/title`);
  assert.ok(milestones.has(issue.milestone), `${issue.id}: unknown milestone`);
  assert.ok(project.fields.Priority.includes(issue.priority), `${issue.id}: invalid priority`);
  assert.ok(project.fields.Area.includes(issue.area), `${issue.id}: invalid area`);
  assert.ok(project.fields.Status.includes(issue.status), `${issue.id}: invalid status`);
  assert.ok(issue.estimate_days === null || (Number.isFinite(issue.estimate_days) && issue.estimate_days > 0), `${issue.id}: invalid estimate`);
  assert.ok(issue.acceptance_criteria.length > 0 && issue.acceptance_criteria.every(s => typeof s === 'string' && s.trim()), `${issue.id}: missing acceptance criteria`);
  for (const label of issue.labels) assert.ok(labels.has(label), `${issue.id}: unknown label ${label}`);
  for (const dep of issue.depends_on) assert.ok(ids.has(dep) && dep !== issue.id, `${issue.id}: invalid dependency ${dep}`);
}
const visited = new Set();
const visiting = new Set();
function visit(id) {
  assert.ok(!visiting.has(id), `Dependency cycle at ${id}`);
  if (visited.has(id)) return;
  visiting.add(id);
  for (const dep of byId.get(id).depends_on) visit(dep);
  visiting.delete(id);
  visited.add(id);
}
for (const id of ids) visit(id);
console.log(`Planning valid: ${issues.length} issues, ${milestones.size} milestones; no missing dependencies or cycles.`);
