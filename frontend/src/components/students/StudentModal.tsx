'use client'

import React from 'react'
import { Student } from '@/types/student'
import { StudentForm } from './StudentForm'

interface StudentModalProps {
  isOpen: boolean
  onClose: () => void
  onSuccess: () => void
  student?: Student
}

export function StudentModal({ isOpen, onClose, onSuccess, student }: StudentModalProps) {
  if (!isOpen) return null

  return (
    <StudentForm
      student={student}
      onClose={onClose}
      onSuccess={() => {
        onSuccess()
        onClose()
      }}
    />
  )
}