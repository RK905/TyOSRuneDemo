import http from 'node:http';
import {readFile} from 'node:fs/promises';
import {solve} from './engine.mjs';
const graph=JSON.parse(await readFile(new URL('../data/network.json',import.meta.url),'utf8'));
const assets=new Map(await Promise.all([['/','index.html','text/html'],['/app.js','app.js','text/javascript'],['/style.css','style.css','text/css']].map(async ([path,file,type]) => [path,{body:await readFile(new URL(`../web/${file}`,import.meta.url)),type}])));
const server=http.createServer(async (req,res) => {
  const send=(status,body,type='application/json') => {res.writeHead(status,{'Content-Type':type,'Cache-Control':'no-store','X-Content-Type-Options':'nosniff'});res.end(type==='application/json'?JSON.stringify(body):body);};
  const path=new URL(req.url,'http://localhost').pathname;
  if(req.method==='GET' && assets.has(path)) {const a=assets.get(path);return send(200,a.body,a.type);}
  if(req.method==='GET' && path==='/api/network') return send(200,graph);
  if(req.method==='GET' && path==='/health') return send(200,{status:'ok',engine:'javascript-reference'});
  if(req.method==='POST' && path==='/api/route') {
    let body='';
    try {
      for await(const chunk of req) {body+=chunk;if(Buffer.byteLength(body)>65536) {send(413,{error:'Body exceeds 64 KB'});return;}}
      let request;try {request=JSON.parse(body);}catch{return send(400,{error:'Invalid JSON'});}
      try {return send(200,solve(graph,request));}catch(error){return send(422,{error:error.message});}
    }catch{return send(400,{error:'Incomplete request'});}
  }
  send(404,{error:'Not found'});
});
const port=Number(process.env.PORT||8080);
server.listen(port,'127.0.0.1',()=>console.log(`Sustain Route · JavaScript reference engine · http://127.0.0.1:${port}`));
