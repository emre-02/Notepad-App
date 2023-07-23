use std::io;

struct Note {
    name: String,
}

impl Note {
    fn add_note(name: String, notes: &mut Vec<Note>) {
        let note = Note { name };
        notes.push(note);
    }
}

fn main() {
    let mut notes: Vec<Note> = Vec::new();

    println!("1- Write a note :");
    println!("2- Add note :");
    println!("3- List notes :"); 
    println!("4- Edit note :"); 
    println!("5- Exit :");

    loop {
        println!("Make your choice :");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Invalid choice");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid");
                continue;
            }
        };

        if choice == 1 {
            let mut note = String::new();
            println!("Note :");
            io::stdin()
                .read_line(&mut note)
                .expect("Invalid");

            let new_note = Note { name: note.trim().to_string() };
            notes.push(new_note);
        } else if choice == 2 {
            let mut new_note = String::new();
            println!("Note :");
            io::stdin()
                .read_line(&mut new_note)
                .expect("Invalid");

            Note::add_note(new_note.trim().to_string(), &mut notes);
        } else if choice == 3 {
            println!("Notes:");
            for (index, note) in notes.iter().enumerate() {
                println!("{}. {}", index + 1, note.name);
            }
        } else if choice == 4 {
            let mut note_index = String::new();
            println!("Enter the index of the note to edit:");
            io::stdin()
                .read_line(&mut note_index)
                .expect("Invalid");

            let index: usize = match note_index.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid index");
                    continue;
                }
            };

            if index >= 1 && index <= notes.len() {
                let mut new_note = String::new();
                println!("Enter the new note:");
                io::stdin()
                    .read_line(&mut new_note)
                    .expect("Invalid");

                notes[index - 1].name = new_note.trim().to_string();
            } else {
                println!("Invalid index");
            }
        } else if choice == 5 {
            println!("Exiting....");
            break;
        } else {
            println!("Invalid choice");
        }
    }
}