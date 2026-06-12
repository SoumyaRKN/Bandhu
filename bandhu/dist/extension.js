"use strict";var y=Object.create;var r=Object.defineProperty;var x=Object.getOwnPropertyDescriptor;var w=Object.getOwnPropertyNames;var C=Object.getPrototypeOf,B=Object.prototype.hasOwnProperty;var M=(t,e)=>{for(var s in e)r(t,s,{get:e[s],enumerable:!0})},m=(t,e,s,o)=>{if(e&&typeof e=="object"||typeof e=="function")for(let n of w(e))!B.call(t,n)&&n!==s&&r(t,n,{get:()=>e[n],enumerable:!(o=x(e,n))||o.enumerable});return t};var v=(t,e,s)=>(s=t!=null?y(C(t)):{},m(e||!t||!t.__esModule?r(s,"default",{value:t,enumerable:!0}):s,t)),k=t=>m(r({},"__esModule",{value:!0}),t);var P={};M(P,{activate:()=>E,deactivate:()=>D});module.exports=k(P);var i=v(require("vscode"));var d=v(require("vscode")),p=class{item;constructor(){this.item=d.window.createStatusBarItem(d.StatusBarAlignment.Left,100),this.item.command="bandhu.helloWorld",this.item.show()}setBusy(){this.item.text="$(loading~spin) Bandhu",this.item.tooltip="Working"}setIdle(){this.item.text="$(check) Bandhu",this.item.tooltip="Ready"}setError(){this.item.text="$(error) Bandhu",this.item.tooltip="Error"}dispose(){this.item.dispose()}};var a=v(require("vscode")),c=class{panel;_onDidReceiveMessage=new a.EventEmitter;onDidReceiveMessage=this._onDidReceiveMessage.event;create(e=a.ViewColumn.One){if(this.panel){this.panel.reveal(e);return}this.panel=a.window.createWebviewPanel("bandhuChat","Bandhu Chat",e,{enableScripts:!0}),this.panel.webview.html=this.getHtml(),this.panel.webview.onDidReceiveMessage(s=>{this._onDidReceiveMessage.fire(s)}),this.panel.onDidDispose(()=>{this.panel=void 0})}append(e){this.panel&&this.panel.webview.postMessage({type:"message",data:e})}clear(){this.panel&&(this.panel.webview.html=this.getHtml())}dispose(){this._onDidReceiveMessage.dispose(),this.panel&&this.panel.dispose()}escapeHtml(e){return e.replace(/[&<>"']/g,s=>({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"})[s]||s)}getHtml(){return`<!DOCTYPE html>
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
        <input id="input" type="text" placeholder="Ask Bandhu..." autocomplete="off" />
        <button id="send">Send</button>
    </div>
    <script>
        const vscode = acquireVsCodeApi();
        const messages = document.getElementById('messages');
        const input = document.getElementById('input');
        const sendBtn = document.getElementById('send');

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
                } else if (data.type === 'response' || data.type === 'tool_result' || data.type === 'tool_error' || data.type === 'error') {
                    addMessage(data.type, data.content || data.error || '');
                }
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
</html>`}};function g(){return{backendUrl:process.env.BANDHU_BACKEND_URL||"http://127.0.0.1:3000",defaultApproval:process.env.BANDHU_DEFAULT_APPROVAL==="true",approvalTimeoutSecs:parseInt(process.env.BANDHU_APPROVAL_TIMEOUT_SECS||"300",10),forbiddenCommands:(process.env.BANDHU_FORBIDDEN_CMDS||"").split(",").map(t=>t.trim().toLowerCase()).filter(Boolean),forbiddenPaths:(process.env.BANDHU_FORBIDDEN_PATHS||"").split(",").map(t=>t.trim()).filter(Boolean)}}var u=g();async function h(t){let e=await fetch(`${u.backendUrl}/chat`,{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({prompt:t})}),s=await e.json();if(!e.ok)throw new Error(`chat failed: ${e.status}`);return s}async function f(t){return(await fetch(`${u.backendUrl}/approve`,{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({request_id:t.id,approved:!0})})).ok}async function b(t){return(await fetch(`${u.backendUrl}/approve`,{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({request_id:t.id,approved:!1})})).ok}var l=class{constructor(e){this.ctx=e;e.subscriptions.push(this)}ctx;status=new p;chat=new c;async activate(){this.chat.create();let e=[];e.push(i.commands.registerCommand("bandhu.helloWorld",()=>this.chat.create())),e.push(i.commands.registerCommand("bandhu.send",async()=>{let s=await i.window.showInputBox({prompt:"Ask Bandhu"});if(s){this.status.setBusy();try{let o=await h(s);this.status.setIdle(),this.show(o)}catch(o){this.status.setError(),this.chat.append({type:"error",error:String(o)})}}})),e.push(this.chat.onDidReceiveMessage(s=>this.handleWebviewMsg(s)));for(let s of e)this.ctx.subscriptions.push(s)}async handleWebviewMsg(e){if(e.type==="send"&&e.text){this.status.setBusy();try{let s=await h(e.text);this.status.setIdle(),this.show(s)}catch(s){this.status.setError(),this.chat.append({type:"error",error:String(s)})}}e.type==="approve"&&e.id&&await f({id:e.id,tool:"",input:{}}),e.type==="reject"&&e.id&&await b({id:e.id,tool:"",input:{}})}show(e){let s=e.messages&&e.messages.length>0?e.messages:[{type:"response",content:e.response}];for(let o of s)this.chat.append(o)}dispose(){this.status.dispose(),this.chat.dispose()}};function E(t){new l(t).activate()}function D(){}0&&(module.exports={activate,deactivate});
