import { NextRequest, NextResponse } from 'next/server'
import { getServerSession } from 'next-auth'

export async function GET(req: NextRequest) {
  // Get session to verify authentication
  const session = await getServerSession()
  if (!session?.user) {
    return NextResponse.json({ error: 'Unauthorized' }, { status: 401 })
  }

  // Extract query parameters
  const url = new URL(req.url)
  const page = parseInt(url.searchParams.get('page') || '1')
  const limit = 10 // Items per page
  const offset = (page - 1) * limit

  try {
    // Fetch students from backend API
    const apiUrl = `${process.env.NEXT_PUBLIC_API_URL}/v1/students?limit=${limit}&offset=${offset}`
    const response = await fetch(apiUrl, {
      headers: {
        'Authorization': `Bearer ${session.user.accessToken}`
      }
    })

    if (!response.ok) {
      throw new Error(`Backend API error: ${response.status}`)
    }

    const data = await response.json()
    return NextResponse.json(data)
  } catch (error) {
    console.error('Error fetching students:', error)
    return NextResponse.json(
      { error: 'Failed to fetch students' },
      { status: 500 }
    )
  }
}

export async function POST(req: NextRequest) {
  // Get session to verify authentication
  const session = await getServerSession()
  if (!session?.user) {
    return NextResponse.json({ error: 'Unauthorized' }, { status: 401 })
  }

  try {
    const studentData = await req.json()

    // Forward the request to the backend API
    const apiUrl = `${process.env.NEXT_PUBLIC_API_URL}/v1/students`
    const response = await fetch(apiUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${session.user.accessToken}`
      },
      body: JSON.stringify(studentData)
    })

    if (!response.ok) {
      const errorData = await response.json()
      return NextResponse.json(
        errorData,
        { status: response.status }
      )
    }

    const data = await response.json()
    return NextResponse.json(data, { status: 201 })
  } catch (error) {
    console.error('Error creating student:', error)
    return NextResponse.json(
      { error: 'Failed to create student' },
      { status: 500 }
    )
  }
} 