import{j as d,a as f,D as m,A as p,S as h}from"./vendor.2869d34b.js";const g=function(){const o=document.createElement("link").relList;if(o&&o.supports&&o.supports("modulepreload"))return;for(const e of document.querySelectorAll('link[rel="modulepreload"]'))n(e);new MutationObserver(e=>{for(const t of e)if(t.type==="childList")for(const r of t.addedNodes)r.tagName==="LINK"&&r.rel==="modulepreload"&&n(r)}).observe(document,{childList:!0,subtree:!0});function s(e){const t={};return e.integrity&&(t.integrity=e.integrity),e.referrerpolicy&&(t.referrerPolicy=e.referrerpolicy),e.crossorigin==="use-credentials"?t.credentials="include":e.crossorigin==="anonymous"?t.credentials="omit":t.credentials="same-origin",t}function n(e){if(e.ep)return;e.ep=!0;const t=s(e);fetch(e.href,t)}};g();const y="modulepreload",l={},v="/",j=function(o,s){return!s||s.length===0?o():Promise.all(s.map(n=>{if(n=`${v}${n}`,n in l)return;l[n]=!0;const e=n.endsWith(".css"),t=e?'[rel="stylesheet"]':"";if(document.querySelector(`link[href="${n}"]${t}`))return;const r=document.createElement("link");if(r.rel=e?"stylesheet":y,e||(r.as="script",r.crossOrigin=""),r.href=n,document.head.appendChild(r),e)return new Promise((u,a)=>{r.addEventListener("load",u),r.addEventListener("error",a)})})).then(()=>o())},c=d,E=f,L=()=>c(m,{children:c(p,{path:"/ringing/:doorbell_id",getComponent:()=>j(()=>import("./Ringing.ade0c845.js"),["assets/Ringing.ade0c845.js","assets/Ringing.445c4a85.css","assets/vendor.2869d34b.js"]).then(i=>i.Ringing)})});h(c(L,{}),document.getElementById("app"));export{E as a,c as j};