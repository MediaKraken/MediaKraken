/*! Video.js | https://videojs.org/help */
import{n as e,r as t}from"./ui-element-CVn6-byi.js";import{r as n}from"./shadow-styles-BY-gO9zn.js";import{t as r}from"./safe-define-DXxmf7BV.js";import{r as i,t as a}from"./create-player-DNDuSABk.js";import"./ui/container.js";import"./background-video-BYzTMKMo.js";const{PlayerElement:o,PlayerController:s}=a({features:i});var c=class extends o{static{this.tagName=`background-video-player`}};r(c);var l=`background-video-player{display:contents}background-video-skin{--media-object-fit:cover;object-fit:var(--media-object-fit);width:100%;min-width:300px;height:100%;min-height:150px;display:block;position:relative}background-video-skin>:not(img,picture){object-fit:var(--media-object-fit);width:100%;height:100%;position:absolute;inset:0}background-video-skin>img,background-video-skin>picture{object-fit:var(--media-object-fit);width:100%;height:100%}
`;function u(){return`
    <media-container>
      <!-- @deprecated slot="media" is no longer required, use the default slot instead -->
      <slot name="media"></slot>
      <slot></slot>
    </media-container>
  `}var d=class r extends t(e){static{this.tagName=`background-video-skin`}static{this.shadowRootOptions={mode:`open`}}static{this.getTemplateHTML=u}constructor(){super(),n(`__media-background-styles`,l),this.shadowRoot||(this.attachShadow(r.shadowRootOptions),this.shadowRoot.innerHTML=u())}};r(d);
//# sourceMappingURL=background.js.map