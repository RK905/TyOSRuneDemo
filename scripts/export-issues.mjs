import {mkdir, readFile, writeFile} from 'node:fs/promises';
import './check-planning.mjs';

const root = new URL('../', import.meta.url);
const {issues} = JSON.parse(await readFile(new URL('planning/backlog.json', root), 'utf8'));
const project = JSON.parse(await readFile(new URL('planning/project.json', root), 'utf8'));
const output = new URL('planning/generated/', root);
await mkdir(output, {recursive: true});
const manifest = [];
for (const issue of issues) {
  const body = `Planning ID: ${issue.id}\n\n${issue.description}\n\n## Acceptance criteria\n\n${issue.acceptance_criteria.map(c => `- [ ] ${c}`).join('\n')}\n\n## Dependencies\n\n${issue.depends_on.join(', ') || 'None'}. Replace planning IDs with GitHub issue links after publication.\n\n## Planning\n\nMilestone: ${issue.milestone}\nPriority: ${issue.priority}\nArea: ${issue.area}\nEstimate: ${issue.estimate_days === null ? 'Discovery required' : `${issue.estimate_days} focused engineer-days`}\n`;
  await writeFile(new URL(`${issue.id}.md`, output), body, 'utf8');
  manifest.push({id: issue.id, title: `[${issue.id}] ${issue.title}`, body_file: `planning/generated/${issue.id}.md`, labels: issue.labels, milestone: project.milestones.find(m => m.id === issue.milestone).title, depends_on: issue.depends_on});
}
await writeFile(new URL('manifest.json', output), JSON.stringify(manifest, null, 2) + '\n', 'utf8');
console.log(`Exported ${issues.length} local issue bodies and manifest to planning/generated/. No remote changes made.`);
