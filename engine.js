// engine.js
const { Marp } = require('@marp-team/marp-core');
const Prism = require('prismjs');
const loadLanguages = require('prismjs/components/');
loadLanguages(['rust']);


module.exports = (opts) => {
  const marp = new Marp(opts)

  marp.highlighter = (code, lang) => {
    return Prism.highlight(code, Prism.languages.rust, lang);
  }

  return marp
}