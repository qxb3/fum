import{a1 as T,a2 as m,a3 as H,_ as D,z as W,y as Y,A as j,a4 as B,a0 as I,i as $,a5 as z,j as w,a6 as P,P as L,m as g,s as R,q as h,T as V,a7 as N,R as A,k as J,a8 as Q,Q as G,a9 as K,aa as U,ab as X,ac as Z,o as x,F as ee,c as te,J as re}from"./DFNjw4_i.js";import{b as ae}from"./BRmKbYT_.js";const ne=["touchstart","touchmove"];function ie(e){return ne.includes(e)}function se(e){var t=H,a=D;T(null),m(null);try{return e()}finally{T(t),m(a)}}const oe=new Set,C=new Set;function ue(e,t,a,i={}){function s(r){if(i.capture||E.call(t,r),!r.cancelBubble)return se(()=>a==null?void 0:a.call(this,r))}return e.startsWith("pointer")||e.startsWith("touch")||e==="wheel"?Y(()=>{t.addEventListener(e,s,i)}):t.addEventListener(e,s,i),s}function _e(e,t,a,i,s){var r={capture:i,passive:s},f=ue(e,t,a,r);(t===document.body||t===window||t===document)&&W(()=>{t.removeEventListener(e,f,r)})}function E(e){var O;var t=this,a=t.ownerDocument,i=e.type,s=((O=e.composedPath)==null?void 0:O.call(e))||[],r=s[0]||e.target,f=0,y=e.__root;if(y){var d=s.indexOf(y);if(d!==-1&&(t===document||t===window)){e.__root=t;return}var v=s.indexOf(t);if(v===-1)return;d<=v&&(f=d)}if(r=s[f]||e.target,r!==t){j(e,"currentTarget",{configurable:!0,get(){return r||a}});var S=H,l=D;T(null),m(null);try{for(var n,o=[];r!==null;){var c=r.assignedSlot||r.parentNode||r.host||null;try{var _=r["__"+i];if(_!==void 0&&(!r.disabled||e.target===r))if(B(_)){var[F,...M]=_;F.apply(r,[e,...M])}else _.call(r,e)}catch(b){n?o.push(b):n=b}if(e.cancelBubble||c===t||c===null)break;r=c}if(n){for(let b of o)queueMicrotask(()=>{throw b});throw n}}finally{e.__root=t,delete e.currentTarget,T(S),m(l)}}}let u;function fe(){u=void 0}function he(e){let t=null,a=w;var i;if(w){for(t=h,u===void 0&&(u=V(document.head));u!==null&&(u.nodeType!==8||u.data!==P);)u=L(u);u===null?g(!1):u=R(L(u))}w||(i=document.head.appendChild(I()));try{$(()=>e(i),z)}finally{a&&(g(!0),u=h,R(t))}}function ye(e,t){var a=t==null?"":typeof t=="object"?t+"":t;a!==(e.__t??(e.__t=e.nodeValue))&&(e.__t=a,e.nodeValue=a+"")}function le(e,t){return q(e,t)}function ve(e,t){N(),t.intro=t.intro??!1;const a=t.target,i=w,s=h;try{for(var r=V(a);r&&(r.nodeType!==8||r.data!==P);)r=L(r);if(!r)throw A;g(!0),R(r),J();const f=q(e,{...t,anchor:r});if(h===null||h.nodeType!==8||h.data!==Q)throw G(),A;return g(!1),f}catch(f){if(f===A)return t.recover===!1&&K(),N(),U(a),g(!1),le(e,t);throw f}finally{g(i),R(s),fe()}}const p=new Map;function q(e,{target:t,anchor:a,props:i={},events:s,context:r,intro:f=!0}){N();var y=new Set,d=l=>{for(var n=0;n<l.length;n++){var o=l[n];if(!y.has(o)){y.add(o);var c=ie(o);t.addEventListener(o,E,{passive:c});var _=p.get(o);_===void 0?(document.addEventListener(o,E,{passive:c}),p.set(o,1)):p.set(o,_+1)}}};d(X(oe)),C.add(d);var v=void 0,S=Z(()=>{var l=a??t.appendChild(I());return x(()=>{if(r){ee({});var n=te;n.c=r}s&&(i.$$events=s),w&&ae(l,null),v=e(l,i)||{},w&&(D.nodes_end=h),r&&re()}),()=>{var c;for(var n of y){t.removeEventListener(n,E);var o=p.get(n);--o===0?(document.removeEventListener(n,E),p.delete(n)):p.set(n,o)}C.delete(d),l!==a&&((c=l.parentNode)==null||c.removeChild(l))}});return k.set(v,S),v}let k=new WeakMap;function pe(e,t){const a=k.get(e);return a?(k.delete(e),a(t)):Promise.resolve()}export{ve as a,_e as e,he as h,le as m,ye as s,pe as u};
