const fs=require("fs");
const ib=fs.readFileSync(".issue_body_tmp","utf8");
const cb=fs.readFileSync(".comment_body_tmp","utf8");
const mr=fs.readFileSync("src/main.rs","utf8");
let a="",c="",r="",cw="",iw="",rw="",cl="";
try{a=fs.readFileSync(".github/agents/devops-sre.md","utf8")}catch(e){}
try{c=fs.readFileSync("Cargo.toml","utf8")}catch(e){}
try{r=fs.readFileSync("README.md","utf8")}catch(e){}
try{cw=fs.readFileSync(".github/workflows/cicd-devops-loop.yml","utf8")}catch(e){}
try{iw=fs.readFileSync(".github/workflows/issue-resolver.yml","utf8")}catch(e){}
try{rw=fs.readFileSync(".github/workflows/release-sync.yml","utf8")}catch(e){}
try{cl=fs.readFileSync(".github/workflows/issue-cleanup.yml","utf8")}catch(e){}
const um="[ISSUE]:\\n"+ib+"\\n\\n[COMMENT]:\\n"+cb+"\\n\\nDiagnose and fix the issue.";
const sp="Expert Rust/DevOps engineer. Fix issues. Response MUST be raw code only: Rust starts with use/fn, YAML starts with name:. NO text, NO markdown.";
const p=JSON.stringify({model:"minimax-m2.7:cloud",stream:false,messages:[{role:"system",content:sp},{role:"user",content:um}]});
fs.writeFileSync("payload.json",p);
