'use client'

import { useSession } from 'next-auth/react'
import { useRouter } from 'next/navigation'
import { useEffect } from 'react'
import Link from 'next/link'
import { AcademicCapIcon, UserGroupIcon, ClipboardDocumentListIcon, ShieldCheckIcon } from '@heroicons/react/24/outline'

export default function HomePage() {
  const { data: session, status } = useSession()
  const router = useRouter()

  useEffect(() => {
    if (status === 'authenticated') {
      // User is logged in, redirect to dashboard
      router.push('/dashboard')
    }
  }, [status, router])

  // Show loading state while checking authentication
  if (status === 'loading') {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600 mx-auto"></div>
          <p className="mt-4 text-gray-600">Loading...</p>
        </div>
      </div>
    )
  }

  // Show landing page for unauthenticated users
  return (
    <div className="min-h-screen bg-gradient-to-br from-indigo-50 via-white to-cyan-50">
      {/* Header */}
      <header className="bg-white shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-6">
            <div className="flex items-center">
              <AcademicCapIcon className="h-8 w-8 text-indigo-600" />
              <h1 className="ml-2 text-2xl font-bold text-gray-900">University Registration System</h1>
            </div>
            <div className="flex space-x-4">
              <Link
                href="/auth/login"
                className="text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium"
              >
                Sign In
              </Link>
              <Link
                href="/auth/register"
                className="bg-indigo-600 hover:bg-indigo-700 text-white px-4 py-2 rounded-md text-sm font-medium"
              >
                Get Started
              </Link>
            </div>
          </div>
        </div>
      </header>

      {/* Hero Section */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div className="text-center">
          <h2 className="text-4xl font-extrabold text-gray-900 sm:text-5xl md:text-6xl">
            Streamline Your
            <span className="text-indigo-600"> Academic Journey</span>
          </h2>
          <p className="mt-6 max-w-2xl mx-auto text-xl text-gray-500">
            Manage student registrations, course enrollments, and academic records with our comprehensive university management platform.
          </p>
          <div className="mt-8 flex justify-center space-x-4">
            <Link
              href="/auth/register"
              className="bg-indigo-600 hover:bg-indigo-700 text-white px-8 py-3 rounded-md text-lg font-medium"
            >
              Register Now
            </Link>
            <Link
              href="/auth/login"
              className="bg-white hover:bg-gray-50 text-indigo-600 border border-indigo-600 px-8 py-3 rounded-md text-lg font-medium"
            >
              Sign In
            </Link>
          </div>
        </div>

        {/* Features Section */}
        <div className="mt-20">
          <div className="text-center">
            <h3 className="text-3xl font-extrabold text-gray-900">
              Everything you need to manage academic operations
            </h3>
            <p className="mt-4 text-lg text-gray-500">
              Our platform provides comprehensive tools for students, faculty, and administrators.
            </p>
          </div>

          <div className="mt-12 grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-4">
            <div className="text-center">
              <div className="flex items-center justify-center h-12 w-12 rounded-md bg-indigo-500 text-white mx-auto">
                <UserGroupIcon className="h-6 w-6" />
              </div>
              <h4 className="mt-4 text-lg font-medium text-gray-900">Student Management</h4>
              <p className="mt-2 text-sm text-gray-500">
                Comprehensive student profiles, enrollment tracking, and academic progress monitoring.
              </p>
            </div>

            <div className="text-center">
              <div className="flex items-center justify-center h-12 w-12 rounded-md bg-indigo-500 text-white mx-auto">
                <ClipboardDocumentListIcon className="h-6 w-6" />
              </div>
              <h4 className="mt-4 text-lg font-medium text-gray-900">Course Registration</h4>
              <p className="mt-2 text-sm text-gray-500">
                Streamlined course enrollment, schedule management, and prerequisite tracking.
              </p>
            </div>

            <div className="text-center">
              <div className="flex items-center justify-center h-12 w-12 rounded-md bg-indigo-500 text-white mx-auto">
                <AcademicCapIcon className="h-6 w-6" />
              </div>
              <h4 className="mt-4 text-lg font-medium text-gray-900">Academic Records</h4>
              <p className="mt-2 text-sm text-gray-500">
                Secure grade management, transcript generation, and academic history tracking.
              </p>
            </div>

            <div className="text-center">
              <div className="flex items-center justify-center h-12 w-12 rounded-md bg-indigo-500 text-white mx-auto">
                <ShieldCheckIcon className="h-6 w-6" />
              </div>
              <h4 className="mt-4 text-lg font-medium text-gray-900">Secure & Reliable</h4>
              <p className="mt-2 text-sm text-gray-500">
                Enterprise-grade security with role-based access control and data protection.
              </p>
            </div>
          </div>
        </div>

        {/* CTA Section */}
        <div className="mt-20 bg-indigo-600 rounded-lg shadow-xl">
          <div className="px-6 py-12 sm:px-12 sm:py-16 lg:px-16">
            <div className="text-center">
              <h3 className="text-3xl font-extrabold text-white">
                Ready to get started?
              </h3>
              <p className="mt-4 text-lg text-indigo-100">
                Join thousands of students and faculty using our platform to streamline academic operations.
              </p>
              <div className="mt-8">
                <Link
                  href="/auth/register"
                  className="bg-white hover:bg-gray-50 text-indigo-600 px-8 py-3 rounded-md text-lg font-medium"
                >
                  Create Your Account
                </Link>
              </div>
            </div>
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="bg-white border-t border-gray-200 mt-20">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          <div className="text-center text-gray-500">
            <p>&copy; 2024 University Registration System. All rights reserved.</p>
          </div>
        </div>
      </footer>
    </div>
  )
}
