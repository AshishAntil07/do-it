use data::lesson::read_lessons;
use shared::AppState;

pub fn search_lesson(
  state: &AppState,
  id: Option<&String>,
  query: Option<&String>,
) -> Result<(), String> {
  let lessons = read_lessons(state)?;

  let filtered = lessons.into_iter().filter(|(lesson_id, lesson)| {
    if state.debug {
      println!(
        "{} - {}, {}",
        lesson_id,
        id.is_none() || *lesson_id == *id.unwrap(),
        query.is_none() || lesson.lesson.contains(query.unwrap())
      );
    }

    return { id.is_none() || *lesson_id == *id.unwrap() } || {
      query.is_none() || lesson.lesson.contains(query.unwrap())
    };
  });

  for (_, lesson) in filtered {
    println!("{}", String::from(lesson));
  }

  Ok(())
}
