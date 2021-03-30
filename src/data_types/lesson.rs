use std::collections::HashSet;

use crate::data_types::*;
use itertools::iproduct;
use serde::Serialize;

#[derive(Debug)]
pub struct LessonPossible<'a> {
    pub student_group: &'a StudentGroup,
    pub subject: &'a Subject,
    pub possible_teachers: Vec<&'a Teacher>,
    pub possible_classrooms: Vec<&'a Classroom>,
    pub possible_dates: DateList,
}

impl<'a> LessonPossible<'a> {
    pub fn new(
        student_group: &'a StudentGroup,
        subject: &'a Subject,
        all_teachers: &'a Vec<Teacher>,
        all_classrooms: &'a Vec<Classroom>,
    ) -> Self {
        Self {
            student_group,
            subject,

            possible_teachers: all_teachers
                .iter()
                .filter(|x| x.can_teach.contains(&subject.name))
                .collect(),

            possible_classrooms: all_classrooms
                .iter()
                .filter(|x| x.capacity >= student_group.size)
                .collect(),

            possible_dates: DateList::new(/*HashSet::new(),*/ 2),
        }
    }

    pub fn cartesian_product_iter(&self) -> impl Iterator<Item = Lesson> + Clone {
        iproduct!(
            self.possible_teachers.iter(),
            self.possible_classrooms.iter(),
            self.possible_dates.clone()
        )
        .map(|x| (*x.0, *x.1, x.2))
        .map(move |x| (self.student_group, self.subject, x))
        .map(Into::<Lesson>::into)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Lesson<'a> {
    pub student_group: &'a StudentGroup,
    pub subject: &'a Subject,
    pub teacher: &'a Teacher,
    pub classroom: &'a Classroom,
    pub date: SimpleDate,
}

type LessonRawPossibility<'a> = (
    &'a StudentGroup,
    &'a Subject,
    (&'a Teacher, &'a Classroom, SimpleDate),
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
