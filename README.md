# Rust Report Card Generator 🎓

A simple and interactive **Rust-based command-line application** that collects student details and subject-wise marks, computes total and average scores, assigns grades, and finally generates a well-formatted **PDF report card** with a tabular layout.

---

## ✨ Features

* Prompt-based input for:

  * Student name
  * Number of subjects
  * Each subject's name and marks
* Calculates:

  * Total marks
  * Average marks
  * Grade assignment (A/B/C/D)
* Generates a structured **PDF report card** using `printpdf`
* PDF includes:

  * Student name and subject count
  * Tabular layout of subjects and marks
  * Summary section with total, average, and grade

---

## 🚀 Getting Started

### Prerequisites

Make sure you have Rust and Cargo installed:

```bash
rustup install stable
```

### Run Locally

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/rust-report-card.git
   cd rust-report-card
   ```

2. Run the application:

   ```bash
   cargo run
   ```

3. Follow the on-screen prompts to enter student data.

4. The report card will be saved as `report_card.pdf` in the root directory.

---

## 📃 Sample Output (PDF Format)

```
📄 STUDENT REPORT CARD

Name: Priya Sharma
Subjects: 3

Subject              Marks
----------------------------------------
Math                 95.00
Physics              88.00
English              92.00
----------------------------------------
Total Marks: 275.00
Average: 91.67
Grade: A
```

---

## 🛋️ Dependencies

* [printpdf](https://crates.io/crates/printpdf) - PDF generation
* [text\_io](https://crates.io/crates/text_io) - User input capture

---

## 📚 License

This project is licensed under the MIT License.

---

## ✨ Future Enhancements

* Add school name/logo in PDF
* Save PDF as `<student_name>_report_card.pdf`
* Add timestamp or teacher signature

---
