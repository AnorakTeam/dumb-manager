export interface Note {
  id: string;
  title: string;
  content: string;
  is_checklist: boolean;
  created_at: string;
  updated_at: string;
}

export interface NoteInput {
  id?: string;
  title: string;
  content: string;
  is_checklist: boolean;
}
