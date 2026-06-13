"use strict";var I=Object.create;var v=Object.defineProperty;var V=Object.getOwnPropertyDescriptor;var $=Object.getOwnPropertyNames;var q=Object.getPrototypeOf,j=Object.prototype.hasOwnProperty;var W=(n,e)=>{for(var t in e)v(n,t,{get:e[t],enumerable:!0})},B=(n,e,t,s)=>{if(e&&typeof e=="object"||typeof e=="function")for(let o of $(e))!j.call(n,o)&&o!==t&&v(n,o,{get:()=>e[o],enumerable:!(s=V(e,o))||s.enumerable});return n};var f=(n,e,t)=>(t=n!=null?I(q(n)):{},B(e||!n||!n.__esModule?v(t,"default",{value:n,enumerable:!0}):t,n)),F=n=>B(v({},"__esModule",{value:!0}),n);var K={};W(K,{activate:()=>J,deactivate:()=>X});module.exports=F(K);var O=f(require("vscode"));var b=f(require("vscode")),g=class{item;constructor(){this.item=b.window.createStatusBarItem(b.StatusBarAlignment.Left,100),this.item.command="bandhu.open",this.setidle(),this.item.show()}setbusy(){this.item.text=u("BANDHU_STATUS_BUSY_TEXT","$(loading~spin) Bandhu"),this.item.tooltip=u("BANDHU_STATUS_BUSY_TOOLTIP","Working")}setidle(){this.item.text=u("BANDHU_STATUS_TEXT","$(check) Bandhu"),this.item.tooltip=u("BANDHU_STATUS_TOOLTIP","Ready")}seterror(){this.item.text=u("BANDHU_STATUS_ERROR_TEXT","$(error) Bandhu"),this.item.tooltip=u("BANDHU_STATUS_ERROR_TOOLTIP","Error")}dispose(){this.item.dispose()}};function u(n,e){return process.env[n]||e}var h=f(require("vscode")),w=class{constructor(e="Ask Bandhu..."){this.placeholder=e}placeholder;panel;_onDidReceiveMessage=new h.EventEmitter;onDidReceiveMessage=this._onDidReceiveMessage.event;create(e=h.ViewColumn.One){if(this.panel){this.panel.reveal(e);return}this.panel=h.window.createWebviewPanel("bandhuChat","Bandhu Chat",e,{enableScripts:!0}),this.panel.webview.html=this.getHtml(),this.panel.webview.onDidReceiveMessage(t=>{this._onDidReceiveMessage.fire(t)}),this.panel.onDidDispose(()=>{this.panel=void 0})}focus(){this.create(),this.panel&&this.panel.webview.postMessage({type:"focus"})}append(e){this.panel&&this.panel.webview.postMessage({type:"message",data:e})}clear(){this.panel&&(this.panel.webview.html=this.getHtml())}dispose(){this._onDidReceiveMessage.dispose(),this.panel&&this.panel.dispose()}escapeHtml(e){return e.replace(/[&<>"']/g,t=>({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"})[t]||t)}getHtml(){return`<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Bandhu Chat</title>
    <style>
        body { font-family: var(--vscode-font-family); padding: 10px; color: var(--vscode-foreground); background: var(--vscode-editor-background); margin: 0; }
        #messages { min-height: 200px; max-height: 600px; overflow-y: auto; padding: 8px; }
        .msg { padding: 8px 12px; margin: 4px 0; border-radius: 4px; word-wrap: break-word; }
        .response { background: var(--vscode-editor-inactiveSelectionBackground); }
        .tool_result { background: var(--vscode-textBlockQuote-background); }
        .tool_error { background: var(--vscode-inputValidation-errorBackground); color: var(--vscode-errorForeground); }
        .tool_approval { background: var(--vscode-inputValidation-infoBackground); border: 1px solid var(--vscode-inputValidation-infoBorder); padding: 12px; }
        .error { background: var(--vscode-inputValidation-errorBackground); color: var(--vscode-errorForeground); }
        .complete { background: var(--vscode-editor-snippetFinalTabstop-foreground); padding: 4px; }
        .approval-buttons { margin-top: 8px; }
        .approval-buttons button { margin-right: 8px; padding: 4px 12px; cursor: pointer; }
        .input-box { margin-top: 12px; padding: 8px; display: flex; gap: 8px; }
        .input-box input { flex: 1; padding: 6px; font-family: var(--vscode-font-family); font-size: 13px; }
        .input-box button { padding: 6px 12px; cursor: pointer; }
        .path-display { font-family: var(--vscode-editor-font-family); font-size: 12px; color: var(--vscode-textLink-foreground); margin: 4px 0; }
    </style>
</head>
<body>
    <div id="messages"></div>
    <div class="input-box">
        <input id="input" type="text" placeholder="${this.escapeHtml(this.placeholder)}" autocomplete="off" />
        <button id="send">Send</button>
    </div>
    <script>
        const vscode = acquireVsCodeApi();
        const messages = document.getElementById('messages');
        const input = document.getElementById('input');
        const sendBtn = document.getElementById('send');

        function formatBuild(result, error) {
            if (error) {
                return 'build failed: ' + error;
            }
            if (!result) {
                return 'build finished';
            }
            const summary = result.summary || 'unknown';
            const command = result.command || '';
            return 'build ' + summary + ': ' + command;
        }

        function formattest(result, error) {
            if (error) {
                return 'test failed: ' + error;
            }
            if (!result) {
                return 'test finished';
            }
            const summary = result.summary || 'unknown';
            const command = result.command || '';
            return 'test ' + summary + ': ' + command;
        }

        function addMessage(type, content) {
            const div = document.createElement('div');
            div.className = 'msg ' + type;
            div.textContent = content;
            messages.appendChild(div);
            messages.scrollTop = messages.scrollHeight;
        }

        function addApproval(id, tool, inputVal, diffVal) {
            const div = document.createElement('div');
            div.className = 'msg tool_approval';
            
            const pathDisplay = document.createElement('div');
            pathDisplay.className = 'path-display';
            pathDisplay.textContent = tool + ': ' + (typeof inputVal === 'object' && inputVal !== null ? inputVal.path || inputVal.command : '');
            div.appendChild(pathDisplay);
            
            if (tool === 'writefile' && diffVal) {
                const diffPre = document.createElement('pre');
                diffPre.style.fontSize = '12px';
                diffPre.style.whiteSpace = 'pre-wrap';
                diffPre.style.margin = '4px 0';
                diffPre.style.backgroundColor = 'var(--vscode-editor-lineHighlightBackground)';
                diffPre.style.padding = '8px';
                diffPre.style.borderRadius = '4px';
                diffPre.textContent = diffVal;
                div.appendChild(diffPre);
            } else {
                const contentPre = document.createElement('pre');
                contentPre.style.fontSize = '12px';
                contentPre.style.whiteSpace = 'pre-wrap';
                contentPre.style.margin = '4px 0';
                contentPre.textContent = JSON.stringify(inputVal, null, 2);
                div.appendChild(contentPre);
            }
            
            const buttonsDiv = document.createElement('div');
            buttonsDiv.className = 'approval-buttons';
            
            const approveBtn = document.createElement('button');
            approveBtn.textContent = 'Approve';
            approveBtn.onclick = () => vscode.postMessage({ type: 'approve', id: id });
            buttonsDiv.appendChild(approveBtn);
            
            const rejectBtn = document.createElement('button');
            rejectBtn.textContent = 'Reject';
            rejectBtn.onclick = () => vscode.postMessage({ type: 'reject', id: id });
            buttonsDiv.appendChild(rejectBtn);
            
            div.appendChild(buttonsDiv);
            messages.appendChild(div);
            messages.scrollTop = messages.scrollHeight;
        }

        window.addEventListener('message', event => {
            const msg = event.data;
            if (msg.type === 'message') {
                const data = msg.data;
                if (data.type === 'tool_approval') {
                    addApproval(data.id, data.tool, data.input, data.diff);
                } else if (data.type === 'response' || data.type === 'tool_result' || data.type === 'tool_error' || data.type === 'build_result' || data.type === 'testresult' || data.type === 'error') {
                    const text = data.type === 'build_result'
                        ? formatBuild(data.result, data.error)
                        : data.type === 'testresult'
                        ? formattest(data.result, data.error)
                        : (data.content || data.error || '');
                    addMessage(data.type, text);
                }
            } else if (msg.type === 'focus') {
                input.focus();
            }
        });

        function sendMessage() {
            const text = input.value.trim();
            if (!text) return;
            vscode.postMessage({ type: 'send', text: text });
            input.value = '';
        }

        sendBtn.onclick = sendMessage;
        input.onkeydown = e => { if (e.key === 'Enter') sendMessage(); };
    </script>
</body>
</html>`}};var D=m("BANDHU_CHAT_TIMEOUT_MS",12e4),Y=m("BANDHU_CHAT_RETRIES",2),T=m("BANDHU_CHAT_RETRY_DELAY_MS",500),E=m("BANDHU_COMMAND_TIMEOUT_MS",3e4),S=m("BANDHU_COMMAND_RETRIES",1),R=m("BANDHU_COMMAND_RETRY_DELAY_MS",500),y=process.env.BANDHU_BACKEND_URL||"http://127.0.0.1:3000";async function k(n,e){let t=await x(`${y}/chat`,{prompt:n},D,Y,T,e),s=await t.json();if(!t.ok)throw new Error(`chat failed: ${t.status}`);return s}async function N(n,e,t){let s=await x(`${y}/chat/stream`,{prompt:n},D,0,T,t);if(!s.ok)throw new Error(`chat stream failed: ${s.status}`);if(!s.body)throw new Error("chat stream body missing");let o=s.body.getReader(),r=new TextDecoder,d="",i;for(;;){let a=await o.read();if(a.done)break;d+=r.decode(a.value,{stream:!0});let l=d.split(`

`);d=l.pop()||"";for(let p of l){let A=M(p);A&&(i=A,e(A))}}let c=r.decode();if(c){let a=M(c);a&&(i=a,e(a))}return{response:i?.content||"",messages:i?[i]:[]}}async function U(n){return(await x(`${y}/approve`,{request_id:n.id,approved:!0},E,S,R)).ok}async function P(n){return(await x(`${y}/approve`,{request_id:n.id,approved:!1},E,S,R)).ok}async function x(n,e,t,s,o,r){let d=0,i;for(;d<=s;){if(r?.aborted)throw new Error("request aborted");let c=new AbortController,a=setTimeout(()=>c.abort(),t),l=()=>{c.abort()};r&&r.addEventListener("abort",l);try{let p=await fetch(n,{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(e),signal:c.signal});if(r&&r.removeEventListener("abort",l),clearTimeout(a),p.ok||d>=s)return p}catch(p){if(r&&r.removeEventListener("abort",l),i=p,clearTimeout(a),r?.aborted)throw p}finally{clearTimeout(a)}d+=1,d<=s&&await z(o)}throw i instanceof Error?i:new Error("request failed")}function M(n){let e=n.split(`
`).filter(t=>t.startsWith("data:")).map(t=>t.slice(5).trim()).join(`
`);if(e)return JSON.parse(e)}function m(n,e){let t=process.env[n],s=Number.parseInt(t||"",10);return Number.isFinite(s)?s:e}async function z(n){await new Promise(e=>setTimeout(e,n))}function H(){return{backendUrl:process.env.BANDHU_BACKEND_URL||"http://127.0.0.1:3000",defaultApproval:process.env.BANDHU_DEFAULT_APPROVAL==="true",approvalTimeoutSecs:parseInt(process.env.BANDHU_APPROVAL_TIMEOUT_SECS||"300",10),forbiddenCommands:(process.env.BANDHU_FORBIDDEN_CMDS||"").split(",").map(n=>n.trim().toLowerCase()).filter(Boolean),forbiddenPaths:(process.env.BANDHU_FORBIDDEN_PATHS||"").split(",").map(n=>n.trim()).filter(Boolean),placeholder:process.env.BANDHU_CHAT_PLACEHOLDER||"Ask Bandhu...",streaming:process.env.BANDHU_CHAT_STREAMING!=="false",outputName:process.env.BANDHU_OUTPUT_NAME||"Bandhu",outputShow:process.env.BANDHU_OUTPUT_SHOW!=="false"}}var L=f(require("vscode")),_=class{channel;constructor(e){this.channel=L.window.createOutputChannel(e)}show(){this.channel.show(!0)}log(e){if(e.type==="build_result"){this.logbuild(e);return}if(e.type==="testresult"){this.logtestmsg(e);return}if(e.type!=="tool_result")return;let t=e.id||"",s=e.result;if(s){if(t==="buildtool"){this.logbuild({type:"build_result",result:s});return}t==="testrunner"&&this.logtest(s)}}dispose(){this.channel.dispose()}logtestmsg(e){let t=e.result,s=e.error,o=new Date().toISOString();if(this.channel.appendLine(`[${o}] test`),s){this.channel.appendLine(`error: ${s}`),this.channel.appendLine("");return}t&&this.writesection("test",t)}logbuild(e){let t=e.result,s=e.error,o=new Date().toISOString();if(this.channel.appendLine(`[${o}] build`),s){this.channel.appendLine(`error: ${s}`),this.channel.appendLine("");return}t&&this.writesection("build",t)}logtest(e){let t=new Date().toISOString();this.channel.appendLine(`[${t}] test`),this.writesection("test",e)}writesection(e,t){this.channel.appendLine(`command: ${t.command||""}`),this.channel.appendLine(`directory: ${t.directory||""}`),this.channel.appendLine(`summary: ${t.summary||""}`),t.stdout&&(this.channel.appendLine("stdout:"),this.channel.appendLine(String(t.stdout))),t.stderr&&(this.channel.appendLine("stderr:"),this.channel.appendLine(String(t.stderr))),this.logfailures(t.failures),this.channel.appendLine("")}logfailures(e){if(!(!Array.isArray(e)||e.length===0)){this.channel.appendLine("failures:");for(let t of e)this.channel.appendLine(String(t))}}};var C=class{constructor(e){this.ctx=e;e.subscriptions.push(this)}ctx;status=new g;config=H();chat=new w(this.config.placeholder);report=new _(this.config.outputName);active;async activate(){this.chat.create();let e=[];e.push(O.commands.registerCommand("bandhu.open",()=>this.chat.focus())),e.push(this.chat.onDidReceiveMessage(t=>this.handleWebviewMsg(t)));for(let t of e)this.ctx.subscriptions.push(t)}async handleWebviewMsg(e){if(e.type==="send"&&e.text){this.active&&this.active.abort();let t=new AbortController;this.active=t,this.status.setbusy();try{if(this.config.streaming)await N(e.text,s=>this.handle(s),t.signal);else{let s=await k(e.text,t.signal);this.show(s)}this.active===t&&(this.active=void 0,this.status.setidle())}catch(s){this.active===t&&(this.active=void 0,this.status.seterror(),this.chat.append({type:"error",error:String(s)}))}}e.type==="approve"&&e.id&&await U({id:e.id,tool:"",input:{}}),e.type==="reject"&&e.id&&await P({id:e.id,tool:"",input:{}})}handle(e){this.report.log(e),this.config.outputShow&&(e.type==="build_result"||e.type==="testresult"||e.type==="tool_result")&&this.report.show(),this.chat.append(e)}show(e){let t=e.messages&&e.messages.length>0?e.messages:[{type:"response",content:e.response}];for(let s of t)this.handle(s)}dispose(){this.status.dispose(),this.chat.dispose(),this.report.dispose()}};function J(n){new C(n).activate()}function X(){}0&&(module.exports={activate,deactivate});
