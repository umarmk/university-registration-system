import { NextRequest, NextResponse } from 'next/server'
import { getServerSession } from 'next-auth'

export async function GET(
  req: NextRequest,
  { params }: { params: { id: string } }
) {
  const session = await getServerSession()
  if (!session?.user) {
    return NextResponse.json({ error: 'Unauthorized' }, { status: 401 })
  }

  try {
    const id = params.id
    const apiUrl = `${process.env.NEXT_PUBLIC_API_URL}/v1/students/${id}`
    
    const response = await fetch(apiUrl, {
      headers: {
        'Authorization': `Bearer ${session.user.accessToken}`
      }
    })

    if (!response.ok) {
      if (response.status === 404) {
        return NextResponse.json({ error: 'Student not found' }, { status: 404 })
      }
      throw new Error(`Backend API error: ${response.status}`)
    }

    const data = await response.json()
    return NextResponse.json(data)
  } catch (error) {
    console.error(`Error fetching student ${params.id}:`, error)
    return NextResponse.json(
      { error: 'Failed to fetch student' },
      { status: 500 }
    )
  }
}

export async function PUT(
  req: NextRequest,
  { params }: { params: { id: string } }
) {
  const session = await getServerSession()
  if (!session?.user) {
    return NextResponse.json({ error: 'Unauthorized' }, { status: 401 })
  }

  try {
    const id = params.id
    const studentData = await req.json()
    
    const apiUrl = `${process.env.NEXT_PUBLIC_API_URL}/v1/students/${id}`
    const response = await fetch(apiUrl, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${session.user.accessToken}`
      },
      body: JSON.stringify(studentData)
    })

    if (!response.ok) {
      const errorData = await response.json()
      return NextResponse.json(errorData, { status: response.status })
    }

    const data = await response.json()
    return NextResponse.json(data)
  } catch (error) {
    console.error(`Error updating student ${params.id}:`, error)
    return NextResponse.json(
      { error: 'Failed to update student' },
      { status: 500 }
    )
  }
}

export async function DELETE(
  req: NextRequest,
  { params }: { params: { id: string } }
) {
  const session = await getServerSession()
  if (!session?.user) {
    return NextResponse.json({ error: 'Unauthorized' }, { status: 401 })
  }

  try {
    const id = params.id
    const apiUrl = `${process.env.NEXT_PUBLIC_API_URL}/v1/students/${id}`
    
    const response = await fetch(apiUrl, {
      method: 'DELETE',
      headers: {
        'Authorization': `Bearer ${session.user.accessToken}`
      }
    })

    if (!response.ok) {
      if (response.status === 404) {
        return NextResponse.json({ error: 'Student not found' }, { status: 404 })
      }
      const errorData = await response.json()
      return NextResponse.json(errorData, { status: response.status })
    }

    return NextResponse.json({ success: true })
  } catch (error) {
    console.error(`Error deleting student ${params.id}:`, error)
    return NextResponse.json(
      { error: 'Failed to delete student' },
      { status: 500 }
    )
  }
} 