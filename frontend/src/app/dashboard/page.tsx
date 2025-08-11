'use client'

import React, { useState } from 'react'
import { useSession } from 'next-auth/react'
import { StudentList } from '@/components/students/StudentList'
import { StudentModal } from '@/components/students/StudentModal'
import { Student } from '@/types/student'
import { useRouter } from 'next/navigation'

export default function DashboardPage() {
  const { data: session, status } = useSession()
  const router = useRouter()
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [selectedStudent, setSelectedStudent] = useState<Student | null>(null)
  const [refreshKey, setRefreshKey] = useState(0)

  // If not authenticated, redirect to login
  if (status === 'unauthenticated') {
    router.push('/auth/login')
    return null
  }   

  // If loading session, show loading state
  if (status === 'loading') {
    return <div className="flex justify-center items-center h-screen">Loading...</div>
  }

  const handleEditStudent = (student: Student) => {
    setSelectedStudent(student)
    setIsModalOpen(true)
  }

  const handleAddStudent = () => {
    setSelectedStudent(null)
    setIsModalOpen(true)
  }

  const handleDeleteStudent = async (id: number) => {
    if (!confirm('Are you sure you want to delete this student?')) return

    try {
      const response = await fetch(`/api/students/${id}`, {
        method: 'DELETE',
        headers: {
          'Authorization': `Bearer ${session?.user?.accessToken}`,
        },
      })

      if (response.ok) {
        // Refresh student list by incrementing the key
        setRefreshKey(prev => prev + 1)
      } else {
        alert('Failed to delete student')
      }
    } catch (error) {
      console.error('Error deleting student:', error)
      alert('An error occurred while deleting the student')
    }
  }

  const handleSuccess = () => {
    setRefreshKey(prev => prev + 1)
  }

  return (
    <div className="min-h-screen bg-gray-100">
      <header className="bg-white shadow">
        <div className="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center">
            <h1 className="text-3xl font-bold text-gray-900">Dashboard</h1>
            <div>
              <button 
                onClick={() => router.push('/auth/logout')}
                className="text-gray-500 hover:text-gray-700 ml-4"
              >
                Logout
              </button>
            </div>
          </div>
        </div>
      </header>
      <main>
        <div className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
          <div className="px-4 py-6 sm:px-0">
            <div className="bg-white shadow overflow-hidden sm:rounded-lg p-4">
              <div className="flex justify-between items-center mb-6">
                <div>
                  <h2 className="text-2xl font-bold text-gray-900">Students</h2>
                  <p className="text-gray-600">Manage student registrations and information</p>
                </div>
                <button
                  onClick={handleAddStudent}
                  className="inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors font-medium shadow-sm"
                >
                  <svg className="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                  </svg>
                  Add Student
                </button>
              </div>
              <StudentList
                key={refreshKey}
                onEdit={handleEditStudent}
                onDelete={handleDeleteStudent}
              />
            </div>
          </div>
        </div>
      </main>

      <StudentModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        onSuccess={handleSuccess}
        student={selectedStudent || undefined}
      />
    </div>
  )
} 