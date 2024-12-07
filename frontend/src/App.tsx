import { useState } from 'react'
import './App.css'

function App() {
  const [formData, setFormData] = useState({ title: '', body: '' })
  const [responseMessage, setResponseMessage] = useState('')
  const [error, setError] = useState('')

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError('')
    try {
      const response = await fetch('http://localhost:8080/issue', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(formData),
      })
      
      const data = await response.json()
      if (!response.ok) {
        throw new Error(data.message || '送信に失敗しました')
      }
      
      setResponseMessage(data.message)
      setFormData({ title: '', body: '' })
    } catch (error) {
      setError(error instanceof Error ? error.message : '予期せぬエラーが発生しました')
      console.error('エラー:', error)
    }
  }

  return (
    <div className="container">
      <form onSubmit={handleSubmit} className="issue-form">
        <div className="form-group">
          <input
            type="text"
            value={formData.title}
            onChange={(e) => setFormData(prev => ({ ...prev, title: e.target.value }))}
            placeholder="イシューのタイトル"
            required
          />
        </div>
        <div className="form-group">
          <textarea
            value={formData.body}
            onChange={(e) => setFormData(prev => ({ ...prev, body: e.target.value }))}
            placeholder="イシューの詳細"
            required
            rows={4}
          />
        </div>
        <button type="submit">イシューを作成</button>
      </form>
      {error && <p className="error-message">{error}</p>}
      {responseMessage && <p className="success-message">{responseMessage}</p>}
    </div>
  )
}

export default App
