import hljs from 'highlight.js/lib/core';
import toml from 'highlight.js/lib/languages/ini';
import bash from './bombadil';
import yaml from 'highlight.js/lib/languages/yaml';

hljs.registerLanguage('bash', bash);
hljs.registerLanguage('yaml', yaml);
hljs.registerLanguage('toml', toml);

export const highlightjsPlugin = () => ({
    name: '@vuepress/plugin-highlightjs',
    async extendsMarkdown(md) {
        md.options.highlight = (code, lang) => {
            if (lang == "text") {
                return code;
            } else {
                return hljs.highlight(code, {language: lang, ignoreIllegals: true}).value
            }
        }
    },
})

