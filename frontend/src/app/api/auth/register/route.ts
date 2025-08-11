import { NextRequest, NextResponse } from 'next/server'

export async function POST(req: NextRequest) {
  try {
    const userData = await req.json()

    // Validate required fields
    const { username, email, password, first_name, last_name } = userData
    if (!username || !email || !password) {
      return NextResponse.json(
        { message: 'Username, email, and password are required' },
        { status: 400 }
      )
    }

    // Forward the request to the backend API
    const apiUrl = `${process.env.NEXT_PUBLIC_API_URL}/auth/register`
    const response = await fetch(apiUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        username,
        email,
        password,
        first_name: first_name || null,
        last_name: last_name || null,
        role_id: 2 // Default to student role (assuming role_id 2 is student)
      })
    })

    const data = await response.json()

    if (!response.ok) {
      return NextResponse.json(
        { message: data.message || 'Registration failed' },
        { status: response.status }
      )
    }

    return NextResponse.json(
      { message: 'Registration successful', user: data.user },
      { status: 201 }
    )
  } catch (error) {
    console.error('Registration error:', error)
    return NextResponse.json(
      { message: 'Internal server error' },
      { status: 500 }
    )
  }
}
