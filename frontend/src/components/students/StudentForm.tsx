'use client'

import React, { useState } from 'react'
import { useForm } from 'react-hook-form'
import { useSession } from 'next-auth/react'
import { NewStudent, Student } from '@/types/student'

interface StudentFormProps {
  student?: Student
  onClose: () => void
  onSuccess: () => void
}

export function StudentForm({ student, onClose, onSuccess }: StudentFormProps) {
  const { data: session } = useSession()
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  
  const { 
    register, 
    handleSubmit, 
    formState: { errors }
  } = useForm<NewStudent>({
    defaultValues: student ? {
      name: student.name,
      email: student.email,
      phone: student.phone,
      course: student.course
    } : undefined
  })

  const onSubmit = async (data: NewStudent) => {
    if (!session?.user?.accessToken) {
      setError('Authentication required')
      return
    }

    setLoading(true)
    setError('')

    try {
      const url = student 
        ? `/api/students/${student.id}` 
        : '/api/students'
      
      const method = student ? 'PUT' : 'POST'

      const response = await fetch(url, {
        method,
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${session.user.accessToken}`
        },
        body: JSON.stringify(data)
      })

      if (response.ok) {
        onSuccess()
      } else {
        const errorData = await response.json()
        setError(errorData.message || 'Failed to save student')
      }
    } catch (err) {
      setError('An error occurred while saving the student')
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
      <div className="bg-white rounded-xl shadow-2xl max-w-md w-full max-h-[90vh] overflow-y-auto">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-gray-200">
          <h2 className="text-xl font-semibold text-gray-900">
            {student ? 'Edit Student' : 'Add New Student'}
          </h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600 transition-colors"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        {/* Form */}
        <form onSubmit={handleSubmit(onSubmit)} className="p-6 space-y-6">
          {/* Name Field */}
          <div className="space-y-2">
            <label htmlFor="name" className="block text-sm font-medium text-gray-700">
              Full Name <span className="text-red-500">*</span>
            </label>
            <input
              id="name"
              type="text"
              placeholder="Enter student's full name"
              {...register('name', { required: 'Name is required' })}
              className={`w-full px-4 py-3 border rounded-lg text-gray-900 placeholder-gray-500 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent ${
                errors.name ? 'border-red-300 bg-red-50' : 'border-gray-300 bg-white hover:border-gray-400'
              }`}
            />
            {errors.name && (
              <p className="text-sm text-red-600 flex items-center">
                <svg className="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
                </svg>
                {errors.name.message}
              </p>
            )}
          </div>

          {/* Email Field */}
          <div className="space-y-2">
            <label htmlFor="email" className="block text-sm font-medium text-gray-700">
              Email Address <span className="text-red-500">*</span>
            </label>
            <input
              id="email"
              type="email"
              placeholder="student@university.edu"
              {...register('email', {
                required: 'Email is required',
                pattern: {
                  value: /^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$/i,
                  message: 'Please enter a valid email address',
                }
              })}
              className={`w-full px-4 py-3 border rounded-lg text-gray-900 placeholder-gray-500 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent ${
                errors.email ? 'border-red-300 bg-red-50' : 'border-gray-300 bg-white hover:border-gray-400'
              }`}
            />
            {errors.email && (
              <p className="text-sm text-red-600 flex items-center">
                <svg className="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
                </svg>
                {errors.email.message}
              </p>
            )}
          </div>

          {/* Phone Field */}
          <div className="space-y-2">
            <label htmlFor="phone" className="block text-sm font-medium text-gray-700">
              Phone Number <span className="text-red-500">*</span>
            </label>
            <input
              id="phone"
              type="tel"
              placeholder="+1 (555) 123-4567"
              {...register('phone', { required: 'Phone number is required' })}
              className={`w-full px-4 py-3 border rounded-lg text-gray-900 placeholder-gray-500 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent ${
                errors.phone ? 'border-red-300 bg-red-50' : 'border-gray-300 bg-white hover:border-gray-400'
              }`}
            />
            {errors.phone && (
              <p className="text-sm text-red-600 flex items-center">
                <svg className="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
                </svg>
                {errors.phone.message}
              </p>
            )}
          </div>

          {/* Course Field */}
          <div className="space-y-2">
            <label htmlFor="course" className="block text-sm font-medium text-gray-700">
              Course/Program <span className="text-red-500">*</span>
            </label>
            <input
              id="course"
              type="text"
              placeholder="e.g., Computer Science, Business Administration"
              {...register('course', { required: 'Course is required' })}
              className={`w-full px-4 py-3 border rounded-lg text-gray-900 placeholder-gray-500 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent ${
                errors.course ? 'border-red-300 bg-red-50' : 'border-gray-300 bg-white hover:border-gray-400'
              }`}
            />
            {errors.course && (
              <p className="text-sm text-red-600 flex items-center">
                <svg className="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
                </svg>
                {errors.course.message}
              </p>
            )}
          </div>

          {/* Error Message */}
          {error && (
            <div className="bg-red-50 border border-red-200 rounded-lg p-4">
              <div className="flex items-center">
                <svg className="w-5 h-5 text-red-400 mr-2" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clipRule="evenodd" />
                </svg>
                <span className="text-sm text-red-700">{error}</span>
              </div>
            </div>
          )}

          {/* Action Buttons */}
          <div className="flex space-x-3 pt-4 border-t border-gray-200">
            <button
              type="button"
              onClick={onClose}
              className="flex-1 px-4 py-3 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 transition-colors font-medium"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={loading}
              className="flex-1 px-4 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium flex items-center justify-center"
            >
              {loading ? (
                <>
                  <svg className="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                  </svg>
                  Saving...
                </>
              ) : (
                student ? 'Update Student' : 'Add Student'
              )}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
} 