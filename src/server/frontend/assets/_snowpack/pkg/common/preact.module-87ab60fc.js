var S,d,M,P,W,H,T={},F=[],Y=/acit|ex(?:s|g|n|p|$)|rph|grid|ows|mnc|ntw|ine[ch]|zoo|^ord|itera/i;function k(_,e){for(var t in e)_[t]=e[t];return _}function I(_){var e=_.parentNode;e&&e.removeChild(_)}function B(_,e,t){var o,l,n,c={};for(n in e)n=="key"?o=e[n]:n=="ref"?l=e[n]:c[n]=e[n];if(arguments.length>2&&(c.children=arguments.length>3?S.call(arguments,2):t),typeof _=="function"&&_.defaultProps!=null)for(n in _.defaultProps)c[n]===void 0&&(c[n]=_.defaultProps[n]);return w(_,c,o,l,null)}function w(_,e,t,o,l){var n={type:_,props:e,key:t,ref:o,__k:null,__:null,__b:0,__e:null,__d:void 0,__c:null,__h:null,constructor:void 0,__v:l??++M};return l==null&&d.vnode!=null&&d.vnode(n),n}function C(_){return _.children}function A(_,e){this.props=_,this.context=e}function x(_,e){if(e==null)return _.__?x(_.__,_.__.__k.indexOf(_)+1):null;for(var t;e<_.__k.length;e++)if((t=_.__k[e])!=null&&t.__e!=null)return t.__e;return typeof _.type=="function"?x(_):null}function O(_){var e,t;if((_=_.__)!=null&&_.__c!=null){for(_.__e=_.__c.base=null,e=0;e<_.__k.length;e++)if((t=_.__k[e])!=null&&t.__e!=null){_.__e=_.__c.base=t.__e;break}return O(_)}}function R(_){(!_.__d&&(_.__d=!0)&&P.push(_)&&!N.__r++||H!==d.debounceRendering)&&((H=d.debounceRendering)||W)(N)}function N(){for(var _;N.__r=P.length;)_=P.sort(function(e,t){return e.__v.__b-t.__v.__b}),P=[],_.some(function(e){var t,o,l,n,c,u;e.__d&&(c=(n=(t=e).__v).__e,(u=t.__P)&&(o=[],(l=k({},n)).__v=n.__v+1,L(u,n,l,t.__n,u.ownerSVGElement!==void 0,n.__h!=null?[c]:null,o,c??x(n),n.__h),K(o,n),n.__e!=c&&O(n)))})}function $(_,e,t,o,l,n,c,u,p,a){var r,v,s,i,f,b,h,y=o&&o.__k||F,m=y.length;for(t.__k=[],r=0;r<e.length;r++)if((i=t.__k[r]=(i=e[r])==null||typeof i=="boolean"?null:typeof i=="string"||typeof i=="number"||typeof i=="bigint"?w(null,i,null,null,i):Array.isArray(i)?w(C,{children:i},null,null,null):i.__b>0?w(i.type,i.props,i.key,null,i.__v):i)!=null){if(i.__=t,i.__b=t.__b+1,(s=y[r])===null||s&&i.key==s.key&&i.type===s.type)y[r]=void 0;else for(v=0;v<m;v++){if((s=y[v])&&i.key==s.key&&i.type===s.type){y[v]=void 0;break}s=null}L(_,i,s=s||T,l,n,c,u,p,a),f=i.__e,(v=i.ref)&&s.ref!=v&&(h||(h=[]),s.ref&&h.push(s.ref,null,i),h.push(v,i.__c||f,i)),f!=null?(b==null&&(b=f),typeof i.type=="function"&&i.__k===s.__k?i.__d=p=j(i,p,_):p=G(_,i,s,y,f,p),typeof t.type=="function"&&(t.__d=p)):p&&s.__e==p&&p.parentNode!=_&&(p=x(s))}for(t.__e=b,r=m;r--;)y[r]!=null&&(typeof t.type=="function"&&y[r].__e!=null&&y[r].__e==t.__d&&(t.__d=x(o,r+1)),X(y[r],y[r]));if(h)for(r=0;r<h.length;r++)Q(h[r],h[++r],h[++r])}function j(_,e,t){for(var o,l=_.__k,n=0;l&&n<l.length;n++)(o=l[n])&&(o.__=_,e=typeof o.type=="function"?j(o,e,t):G(t,o,o,l,o.__e,e));return e}function z(_,e){return e=e||[],_==null||typeof _=="boolean"||(Array.isArray(_)?_.some(function(t){z(t,e)}):e.push(_)),e}function G(_,e,t,o,l,n){var c,u,p;if(e.__d!==void 0)c=e.__d,e.__d=void 0;else if(t==null||l!=n||l.parentNode==null)e:if(n==null||n.parentNode!==_)_.appendChild(l),c=null;else{for(u=n,p=0;(u=u.nextSibling)&&p<o.length;p+=2)if(u==l)break e;_.insertBefore(l,n),c=n}return c!==void 0?c:l.nextSibling}function Z(_,e,t,o,l){var n;for(n in t)n==="children"||n==="key"||n in e||U(_,n,null,t[n],o);for(n in e)l&&typeof e[n]!="function"||n==="children"||n==="key"||n==="value"||n==="checked"||t[n]===e[n]||U(_,n,e[n],t[n],o)}function V(_,e,t){e[0]==="-"?_.setProperty(e,t):_[e]=t==null?"":typeof t!="number"||Y.test(e)?t:t+"px"}function U(_,e,t,o,l){var n;e:if(e==="style")if(typeof t=="string")_.style.cssText=t;else{if(typeof o=="string"&&(_.style.cssText=o=""),o)for(e in o)t&&e in t||V(_.style,e,"");if(t)for(e in t)o&&t[e]===o[e]||V(_.style,e,t[e])}else if(e[0]==="o"&&e[1]==="n")n=e!==(e=e.replace(/Capture$/,"")),e=e.toLowerCase()in _?e.toLowerCase().slice(2):e.slice(2),_.l||(_.l={}),_.l[e+n]=t,t?o||_.addEventListener(e,n?J:q,n):_.removeEventListener(e,n?J:q,n);else if(e!=="dangerouslySetInnerHTML"){if(l)e=e.replace(/xlink[H:h]/,"h").replace(/sName$/,"s");else if(e!=="href"&&e!=="list"&&e!=="form"&&e!=="tabIndex"&&e!=="download"&&e in _)try{_[e]=t??"";break e}catch(c){}typeof t=="function"||(t!=null&&(t!==!1||e[0]==="a"&&e[1]==="r")?_.setAttribute(e,t):_.removeAttribute(e))}}function q(_){this.l[_.type+!1](d.event?d.event(_):_)}function J(_){this.l[_.type+!0](d.event?d.event(_):_)}function L(_,e,t,o,l,n,c,u,p){var a,r,v,s,i,f,b,h,y,m,D,g=e.type;if(e.constructor!==void 0)return null;t.__h!=null&&(p=t.__h,u=e.__e=t.__e,e.__h=null,n=[u]),(a=d.__b)&&a(e);try{e:if(typeof g=="function"){if(h=e.props,y=(a=g.contextType)&&o[a.__c],m=a?y?y.props.value:a.__:o,t.__c?b=(r=e.__c=t.__c).__=r.__E:("prototype"in g&&g.prototype.render?e.__c=r=new g(h,m):(e.__c=r=new A(h,m),r.constructor=g,r.render=_e),y&&y.sub(r),r.props=h,r.state||(r.state={}),r.context=m,r.__n=o,v=r.__d=!0,r.__h=[]),r.__s==null&&(r.__s=r.state),g.getDerivedStateFromProps!=null&&(r.__s==r.state&&(r.__s=k({},r.__s)),k(r.__s,g.getDerivedStateFromProps(h,r.__s))),s=r.props,i=r.state,v)g.getDerivedStateFromProps==null&&r.componentWillMount!=null&&r.componentWillMount(),r.componentDidMount!=null&&r.__h.push(r.componentDidMount);else{if(g.getDerivedStateFromProps==null&&h!==s&&r.componentWillReceiveProps!=null&&r.componentWillReceiveProps(h,m),!r.__e&&r.shouldComponentUpdate!=null&&r.shouldComponentUpdate(h,r.__s,m)===!1||e.__v===t.__v){r.props=h,r.state=r.__s,e.__v!==t.__v&&(r.__d=!1),r.__v=e,e.__e=t.__e,e.__k=t.__k,e.__k.forEach(function(E){E&&(E.__=e)}),r.__h.length&&c.push(r);break e}r.componentWillUpdate!=null&&r.componentWillUpdate(h,r.__s,m),r.componentDidUpdate!=null&&r.__h.push(function(){r.componentDidUpdate(s,i,f)})}r.context=m,r.props=h,r.state=r.__s,(a=d.__r)&&a(e),r.__d=!1,r.__v=e,r.__P=_,a=r.render(r.props,r.state,r.context),r.state=r.__s,r.getChildContext!=null&&(o=k(k({},o),r.getChildContext())),v||r.getSnapshotBeforeUpdate==null||(f=r.getSnapshotBeforeUpdate(s,i)),D=a!=null&&a.type===C&&a.key==null?a.props.children:a,$(_,Array.isArray(D)?D:[D],e,t,o,l,n,c,u,p),r.base=e.__e,e.__h=null,r.__h.length&&c.push(r),b&&(r.__E=r.__=null),r.__e=!1}else n==null&&e.__v===t.__v?(e.__k=t.__k,e.__e=t.__e):e.__e=ee(t.__e,e,t,o,l,n,c,p);(a=d.diffed)&&a(e)}catch(E){e.__v=null,(p||n!=null)&&(e.__e=u,e.__h=!!p,n[n.indexOf(u)]=null),d.__e(E,e,t)}}function K(_,e){d.__c&&d.__c(e,_),_.some(function(t){try{_=t.__h,t.__h=[],_.some(function(o){o.call(t)})}catch(o){d.__e(o,t.__v)}})}function ee(_,e,t,o,l,n,c,u){var p,a,r,v=t.props,s=e.props,i=e.type,f=0;if(i==="svg"&&(l=!0),n!=null){for(;f<n.length;f++)if((p=n[f])&&(p===_||(i?p.localName==i:p.nodeType==3))){_=p,n[f]=null;break}}if(_==null){if(i===null)return document.createTextNode(s);_=l?document.createElementNS("http://www.w3.org/2000/svg",i):document.createElement(i,s.is&&s),n=null,u=!1}if(i===null)v===s||u&&_.data===s||(_.data=s);else{if(n=n&&S.call(_.childNodes),a=(v=t.props||T).dangerouslySetInnerHTML,r=s.dangerouslySetInnerHTML,!u){if(n!=null)for(v={},f=0;f<_.attributes.length;f++)v[_.attributes[f].name]=_.attributes[f].value;(r||a)&&(r&&(a&&r.__html==a.__html||r.__html===_.innerHTML)||(_.innerHTML=r&&r.__html||""))}if(Z(_,s,v,l,u),r)e.__k=[];else if(f=e.props.children,$(_,Array.isArray(f)?f:[f],e,t,o,l&&i!=="foreignObject",n,c,n?n[0]:t.__k&&x(t,0),u),n!=null)for(f=n.length;f--;)n[f]!=null&&I(n[f]);u||("value"in s&&(f=s.value)!==void 0&&(f!==_.value||i==="progress"&&!f)&&U(_,"value",f,v.value,!1),"checked"in s&&(f=s.checked)!==void 0&&f!==_.checked&&U(_,"checked",f,v.checked,!1))}return _}function Q(_,e,t){try{typeof _=="function"?_(e):_.current=e}catch(o){d.__e(o,t)}}function X(_,e,t){var o,l;if(d.unmount&&d.unmount(_),(o=_.ref)&&(o.current&&o.current!==_.__e||Q(o,null,e)),(o=_.__c)!=null){if(o.componentWillUnmount)try{o.componentWillUnmount()}catch(n){d.__e(n,e)}o.base=o.__P=null}if(o=_.__k)for(l=0;l<o.length;l++)o[l]&&X(o[l],e,typeof _.type!="function");t||_.__e==null||I(_.__e),_.__e=_.__d=void 0}function _e(_,e,t){return this.constructor(_,t)}function te(_,e,t){var o,l,n;d.__&&d.__(_,e),l=(o=typeof t=="function")?null:t&&t.__k||e.__k,n=[],L(e,_=(!o&&t||e).__k=B(C,null,[_]),l||T,T,e.ownerSVGElement!==void 0,!o&&t?[t]:l?null:e.firstChild?S.call(e.childNodes):null,n,!o&&t?t:l?l.__e:e.firstChild,o),K(n,_)}function ne(_,e,t){var o,l,n,c=k({},_.props);for(n in e)n=="key"?o=e[n]:n=="ref"?l=e[n]:c[n]=e[n];return arguments.length>2&&(c.children=arguments.length>3?S.call(arguments,2):t),w(_.type,c,o||_.key,l||_.ref,null)}S=F.slice,d={__e:function(_,e){for(var t,o,l;e=e.__;)if((t=e.__c)&&!t.__)try{if((o=t.constructor)&&o.getDerivedStateFromError!=null&&(t.setState(o.getDerivedStateFromError(_)),l=t.__d),t.componentDidCatch!=null&&(t.componentDidCatch(_),l=t.__d),l)return t.__E=t}catch(n){_=n}throw _}},M=0,A.prototype.setState=function(_,e){var t;t=this.__s!=null&&this.__s!==this.state?this.__s:this.__s=k({},this.state),typeof _=="function"&&(_=_(k({},t),this.props)),_&&k(t,_),_!=null&&this.__v&&(e&&this.__h.push(e),R(this))},A.prototype.forceUpdate=function(_){this.__v&&(this.__e=!0,_&&this.__h.push(_),R(this))},A.prototype.render=C,P=[],W=typeof Promise=="function"?Promise.prototype.then.bind(Promise.resolve()):setTimeout,N.__r=0;export{z as A,ne as B,te as S,A as _,C as d,d as l,B as v};
