export interface Student {
  id: number;
  name: string;
  email: string;
  phone: string;
  course: string;
  created_at?: string;
  updated_at?: string;
  created_by?: number;
  updated_by?: number;
}

export interface NewStudent {
  name: string;
  email: string;
  phone: string;
  course: string;
} 