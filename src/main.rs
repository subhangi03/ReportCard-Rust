use printpdf::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use text_io::read;

#[derive(Debug)]
struct Subject {
    name: String,
    marks: f32,
}

#[derive(Debug)]
struct Student {
    name: String,
    subjects: Vec<Subject>,
}

impl Student {
    fn total(&self) -> f32 {
        self.subjects.iter().map(|s| s.marks).sum()
    }

    fn average(&self) -> f32 {
        self.total() / self.subjects.len() as f32
    }

    fn grade(&self) -> &'static str {
        let avg = self.average();
        match avg {
            90.0..=100.0 => "A",
            75.0..=89.9 => "B",
            60.0..=74.9 => "C",
            _ => "D",
        }
    }
}

fn generate_pdf(student: &Student) {
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    let font_mono = doc.add_builtin_font(BuiltinFont::Courier).unwrap();

    let mut y = 270.0;

    // Title
    layer.use_text("📄 STUDENT REPORT CARD", 20.0, Mm(50.0), Mm(y), &font);
    y -= 20.0;

    // Student info
    let info = format!("Name: {}\nSubjects: {}\n", student.name, student.subjects.len());
    for line in info.lines() {
        layer.use_text(line, 14.0, Mm(20.0), Mm(y), &font);
        y -= 8.0;
    }

    y -= 5.0;

    // Table Header
    layer.use_text("Subject", 14.0, Mm(20.0), Mm(y), &font);
    layer.use_text("Marks", 14.0, Mm(130.0), Mm(y), &font);
    y -= 8.0;

    layer.use_text("----------------------------------------", 12.0, Mm(20.0), Mm(y), &font_mono);
    y -= 8.0;

    // Subject data
    for subject in &student.subjects {
        layer.use_text(&subject.name, 12.0, Mm(20.0), Mm(y), &font_mono);
        layer.use_text(&format!("{:.2}", subject.marks), 12.0, Mm(130.0), Mm(y), &font_mono);
        y -= 8.0;
    }

    y -= 10.0;

    // Summary
    layer.use_text("----------------------------------------", 12.0, Mm(20.0), Mm(y), &font_mono);
    y -= 10.0;

    layer.use_text(&format!("Total Marks: {:.2}", student.total()), 14.0, Mm(20.0), Mm(y), &font);
    y -= 8.0;
    layer.use_text(&format!("Average: {:.2}", student.average()), 14.0, Mm(20.0), Mm(y), &font);
    y -= 8.0;
    layer.use_text(&format!("Grade: {}", student.grade()), 14.0, Mm(20.0), Mm(y), &font);

    doc.save(&mut BufWriter::new(File::create("report_card.pdf").unwrap())).unwrap();
    println!("\n✅ PDF generated: 'report_card.pdf'");
}

fn main() {
    println!("📋 Rust Report Card Generator");

    print!("Enter student name: ");
    std::io::stdout().flush().unwrap();
    let name: String = read!("{}\n");

    print!("Enter number of subjects: ");
    std::io::stdout().flush().unwrap();
    let count: usize = read!();

    let mut subjects = Vec::new();
    for i in 1..=count {
        print!("Subject {i} name: ");
        std::io::stdout().flush().unwrap();
        let sub_name: String = read!("{}\n");

        print!("Marks for {sub_name}: ");
        std::io::stdout().flush().unwrap();
        let marks: f32 = read!();

        subjects.push(Subject {
            name: sub_name,
            marks,
        });
    }

    let student = Student { name, subjects };

    println!("\n🎓 Generating report...");
    generate_pdf(&student);
}
