import * as monaco from 'monaco-editor'
import './monaco.css'

function getWorkerName(label) {
    if (label === 'json' || label === 'css' || label === 'html') {
        return label
    }
    if (label === 'typescript' || label === 'javascript') {
        return 'typescript'
    }
    return 'editor'
}

self.MonacoEnvironment = {
    getWorkerUrl(_moduleId, label) {
        return `/static/js/${getWorkerName(label)}.worker.js`
    }
}

const languageMap = {
    'c': 'objective-c', // Somehow the repo doesn't have C language
    'c-plus-plus': 'cpp',
    'c-sharp': 'csharp',
    'haskell': null,
    'html': 'html',
    'java': 'java',
    'javascript': 'javascript',
    'jinja2': null,
    'jsx': 'javascript',
    'markdown': 'markdown',
    'perl': 'perl',
    'perl6': null,
    'php': 'php',
    'postgresql': 'sql',
    'python2': 'python',
    'python3': 'python',
    'rust': 'rust',
    'sh': 'shell',
    'sql': 'sql',
    'sqlite': 'sql',
    'typescript': 'typescript',
    'typescript-jsx': 'typescript',
}

class MonacoEditor {
    constructor(textarea, container, editor) {
        this.textarea = textarea
        this.container = container
        this.editor = editor
    }

    setLanguage(identifier) {
        monaco.editor.setModelLanguage(this.editor.getModel(), languageMap[identifier])
    }

    getValue() {
        return this.editor.getValue()
    }

    setValue(value) {
        this.editor.setValue(value)
    }

    unload() {
        this.editor.dispose()
        this.container.remove()
        this.textarea.style.display = 'inline'
        this.editor.toTextArea()
    }
}

export default function createMonacoEditor(textarea, onChange) {
    const container = document.createElement('div')
    container.className = 'monaco'
    textarea.before(container)
    textarea.style.display = 'none'
    const editor = monaco.editor.create(container, {
        value: textarea.value,
    })
    editor.onDidChangeModelContent(onChange)
    textarea.form.addEventListener('submit', () => textarea.value = editor.getValue())
    return new MonacoEditor(textarea, container, editor)
}
