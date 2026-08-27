import { useState, useEffect, useRef } from 'react'
import { open, save } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import './App.css'

function statusClass(status) {
  if (status.startsWith('Error')) return 'status error'
  if (status === 'Processing...') return 'status processing'
  if (status !== 'Ready') return 'status success'
  return 'status'
}

function App() {
  const [inputPath, setInputPath] = useState('')
  const [outputPath, setOutputPath] = useState('')
  const [status, setStatus] = useState('Ready')
  const [isProcessing, setIsProcessing] = useState(false)
  const [progress, setProgress] = useState({ current: 0, total: 0 })
  const [shouldStart, setShouldStart] = useState(false)
  const unlistenRef = useRef(null)

  useEffect(() => {
    async function setup() {
      unlistenRef.current = await listen('progress', (event) => {
        setProgress(event.payload)
      })
    }
    setup()
    return () => {
      if (unlistenRef.current) {
        unlistenRef.current()
      }
    }
  }, [])

  useEffect(() => {
    if (!shouldStart) return

    let cancelled = false

    ;(async () => {
      try {
        const result = await invoke('transform_images', { input: inputPath, output: outputPath })
        if (!cancelled) setStatus(result)
      } catch (e) {
        if (!cancelled) setStatus('Error: ' + e)
      } finally {
        if (!cancelled) {
          setIsProcessing(false)
          setShouldStart(false)
        }
      }
    })()

    return () => { cancelled = true }
  }, [shouldStart])

  async function selectInputFolder() {
    const selected = await open({ directory: true, multiple: false, title: 'Select input folder' })
    if (selected) {
      setInputPath(selected)
    }
  }

  async function selectOutputFile() {
    const selected = await save({
      title: 'Save output zip',
      defaultPath: 'output.zip',
      filters: [{ name: 'ZIP Archive', extensions: ['zip'] }],
    })
    if (selected) {
      setOutputPath(selected)
    }
  }

  function handleTransformClick() {
    if (!inputPath || !outputPath) {
      setStatus('Error: Select both input folder and output file.')
      return
    }
    setIsProcessing(true)
    setShouldStart(true)
  }

  return (
    <>
      <h1 className="title">Chhobi</h1>
      <p className="subtitle">Bulk Image Resizer</p>

      <div className="field">
        <button className="button" onClick={selectInputFolder}>Select Input Folder</button>
        <span className="path">{inputPath || '(none selected)'}</span>
      </div>

      <div className="field">
        <button className="button" onClick={selectOutputFile}>Select Output File</button>
        <span className="path">{outputPath || '(none selected)'}</span>
      </div>

      <div className="field" style={{ justifyContent: 'center', marginTop: '20px' }}>
        <button className="button button--primary" onClick={handleTransformClick}>Transform Images</button>
      </div>

      <div className={statusClass(status)}>{status}</div>

      {isProcessing && (
        <div className="modal-overlay">
          <div className="modal">
            <div className="modal-header">
              <span className="spinner" />
              <span>Processing Images</span>
            </div>
            <div className="progress-bar">
              <div
                className="progress-bar-fill"
                style={{ width: `${progress.total > 0 ? (progress.current / progress.total) * 100 : 0}%` }}
              />
            </div>
            <div className="modal-status">
              Processing {progress.current} of {progress.total} images...
            </div>
          </div>
        </div>
      )}
    </>
  )
}

export default App