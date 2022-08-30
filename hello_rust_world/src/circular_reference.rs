// student* --- *course -->many to many relation
// Rc (reference counted) wrapper is used to avoid lifetime problem because if student destroyed then in course this student exist that was destoryed!

//RefCell is used to avoid borrowing mutability problem

//normalization
//student
//course
//Vec<Enrollment {course, student }> //now you have all the information for storing association b/w student and course without using the Rc and RefCell

struct Student {
    name: String,
    //course: Vec<Course> //ownership
    //courses: Vec<&'a Course<'a>>, //referenes
}

impl Student {
    //want to get the courses that the student takes
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform
            .enrollments
            .iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}
struct Course {
    name: String,
}

struct Enrollment<'a> {
    //both student and course must be same lifetime so there is need to implement lifetim
    student: &'a Student,
    course: &'a Course,
}
//factory method

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
        Enrollment { student, course }
    }
}

//platform that stored all the things and having no. of enrollments
struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>,
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new(),
        }
    }

    //we will change the set of enrollments that's why &mut self
    fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        self.enrollments.push(Enrollment::new(student, course))
    }
}

pub fn c_r() {
    let maham = Student {
        name: "maham".into(),
        //the Into trait tries to convert the item you feed it to the requested type, assuming the type that's needed is actually known. In this case, into() takes a &str and converts it into a String.
    };
    let course = Course {
        name: "Intro to Rust".into(),
    };
    let course2 = Course {
        name: "Mern".into(),
    };

    //Now, we can build a platform where the courses are held and sold and where students can actually enroll onto a course.

    let mut p = Platform::new();
    p.enroll(&maham, &course);
    p.enroll(&maham, &course2);

    //Now printout all the courses that the maham takes
    for c in maham.courses(p) {
        println!("Maham is taking {}", c);
    }
}
