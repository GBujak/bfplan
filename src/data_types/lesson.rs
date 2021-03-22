use crate::data_types::*;
use itertools::iproduct;

struct LessonPossible<'a> {
    pub student_group: &'a StudentGroup,
    pub subject: &'a Subject,
    pub possible_teachers: Vec<&'a Teacher>,
    pub possible_classrooms: Vec<&'a Classroom>,
    pub possible_dates: Vec<&'a SimpleDate>,
}

impl<'a> LessonPossible<'a> {
    pub fn new(
        student_group: &'a StudentGroup,
        subject: &'a Subject,
        possible_teachers: Vec<&'a Teacher>,
        possible_classrooms: Vec<&'a Classroom>,
        possible_dates: Vec<&'a SimpleDate>,
    ) -> Self {
        Self {
            student_group,
            subject,
            possible_teachers,
            possible_classrooms,
            possible_dates,
        }
    }

    pub fn cartesian_product_iter(&self) -> impl Iterator<Item = Lesson> {
        iproduct!(
            self.possible_teachers.iter(),
            self.possible_classrooms.iter(),
            self.possible_dates.iter()
        )
        .map(|x| (*x.0, *x.1, *x.2))
        .map(move |x| (self.student_group, self.subject, x))
        .map(Into::<Lesson>::into)
    }
}

pub struct Lesson<'a> {
    pub student_group: &'a StudentGroup,
    pub subject: &'a Subject,
    pub teacher: &'a Teacher,
    pub classroom: &'a Classroom,
    pub date: &'a SimpleDate,
}

type LessonRawPossibility<'a> = (
    &'a StudentGroup,
    &'a Subject,
    (&'a Teacher, &'a Classroom, &'a SimpleDate),
);

impl<'a> From<LessonRawPossibility<'a>> for Lesson<'a> {
    fn from(p: LessonRawPossibility<'a>) -> Self {
        Self {
            student_group: p.0,
            subject: p.1,
            teacher: p.2 .0,
            classroom: p.2 .1,
            date: p.2 .2,
        }
    }
}
