import createTextareaEditor from '../editor-types/textarea'
import getLanguage from './get-language'
import Output from './output'
import WrapperButtons from './wrapper-buttons'
import { EditorType, types, getCurrentEditor, onChange } from '../../editor-types'

class Editor {
    languageSelector: HTMLSelectElement
    wrapperButtons: WrapperButtons
    codeElement: HTMLTextAreaElement
    output: Output
    autodeleteText: HTMLSpanElement
    autodeleteCheckbox: HTMLLabelElement
    helloWorldLink: HTMLSpanElement
    submit: HTMLInputElement
    detailsElement: HTMLDetailsElement
    stdinElement: HTMLTextAreaElement
    editor: EditorType
    currentLanguage: string | null = null
    abortEval: AbortController | null = null

    initialize(form) {
        this.languageSelector = form.querySelector('#language')
        this.wrapperButtons = new WrapperButtons(form.querySelector('#wrapper-buttons'), this.run.bind(this))
        this.codeElement = form.querySelector('#code')
        this.initializeEditor(createTextareaEditor)
        onChange(editor => this.changeEditor(editor))
        this.initConfiguredEditor()
        this.output = new Output(form.querySelector('#output'))
        const stdout = document.querySelector<HTMLInputElement>('#dbstdout')
        if (stdout) {
            this.output.display({}, {
                stdout: stdout.value,
                stderr: document.querySelector<HTMLInputElement>('#dbstderr').value,
                status: +document.querySelector<HTMLInputElement>('#dbstatus') ?.value,
            })
        }
        this.autodeleteText = form.querySelector('#autodelete-text')
        this.autodeleteCheckbox = form.querySelector('#automatically-hidden-label')
        this.helloWorldLink = form.querySelector('#hello-world')
        this.submit = form.querySelector('[type=submit]')
        this.submit.disabled = true
        form.addEventListener('submit', () => {
            if (this.output.json && !this.output.wrapper.isFormatter) {
                for (const name of ['stdout', 'stderr', 'status']) {
                    const elem = form.querySelector(`#${name}`) || document.createElement('input')
                    elem.type = 'hidden'
                    elem.name = name
                    elem.value = this.output.json[name]
                    form.append(elem)
                }
            } else {
                this.stdinElement.value = ''
            }
        })
        this.detailsElement = document.createElement('details')
        const summary = document.createElement('summary')
        summary.textContent = 'Standard input'
        this.stdinElement = document.createElement('textarea')
        this.stdinElement.name = 'stdin'
        this.stdinElement.addEventListener('change', () => this.changeToLookLikeNewPaste())
        this.detailsElement.append(summary, this.stdinElement)
        const dbStdin = document.querySelector<HTMLInputElement>('#dbstdin') ?.value
        if (dbStdin) {
            this.stdinElement.value = dbStdin
            this.detailsElement.open = true
        } else {
            this.detailsElement.style.display = 'none'
        }
        form.querySelector('#buttons').append(this.detailsElement)
        if (this.autodeleteText) {
            this.autodeleteCheckbox.style.display = 'none'
        }
        this.assignEvents()
        this.updateLanguage()
    }

    async initConfiguredEditor() {
        this.changeEditor(await types[getCurrentEditor()].createView())
    }

    changeEditor(createEditor) {
        this.editor.unload()
        this.initializeEditor(createEditor)
    }

    initializeEditor(createEditor) {
        this.editor = createEditor(this.codeElement, () => this.changeToLookLikeNewPaste())
        if (this.currentLanguage) {
            this.editor.setLanguage(this.currentLanguage)
        }
    }

    setLanguage(language) {
        this.currentLanguage = language
        this.editor.setLanguage(language)
    }

    changeToLookLikeNewPaste() {
        if (this.autodeleteText) {
            this.autodeleteText.style.display = 'none'
            this.autodeleteCheckbox.style.display = ''
        }
        this.submit.disabled = false
        this.output.clear()
    }

    assignEvents() {
        this.languageSelector.addEventListener('change', () => {
            this.updateLanguage()
            this.changeToLookLikeNewPaste()
        })
    }

    async updateLanguage() {
        this.wrapperButtons.clear()
        this.helloWorldLink.textContent = ''
        const identifier = this.getLanguageIdentifier()
        this.setLanguage(identifier)
        const isStillValid = () => identifier === this.getLanguageIdentifier()
        const language = await getLanguage(identifier, isStillValid)
        // This deals with user changing the language after asynchronous event
        if (isStillValid()) {
            if (language.helloWorldPaste) {
                const anchor = document.createElement('a')
                anchor.href = '/' + language.helloWorldPaste
                anchor.textContent = 'Hello world program'
                this.helloWorldLink.append(' | ', anchor)
            }
            this.detailsElement.style.display = language.implementations.length ? 'block' : 'none'
            this.wrapperButtons.update(language.implementations)
        }
    }

    getLanguageIdentifier() {
        return this.languageSelector.selectedOptions[0].value
    }

    async run(wrapper, compilerOptions) {
        this.output.clear()
        if (this.abortEval) {
            this.abortEval.abort()
        }
        this.abortEval = new AbortController
        const body = new URLSearchParams
        body.append('compilerOptions', compilerOptions)
        body.append('code', this.editor.getValue())
        body.append('stdin', this.stdinElement.value)
        const parameters = {
            method: 'POST',
            body,
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
            },
            signal: this.abortEval.signal,
        }
        const path = `/api/v0/run/${wrapper.identifier}`
        let response
        try {
            response = await (await fetch(path, parameters)).json()
        } catch (e) {
            if (e.name === 'AbortError') {
                return
            }
            this.output.error()
            throw e
        }
        if (wrapper.isFormatter) {
            this.editor.setValue(response.stdout)
        }
        this.output.display(wrapper, response)
    }
}

export default function createEditor(form) {
    new Editor().initialize(form)
}
