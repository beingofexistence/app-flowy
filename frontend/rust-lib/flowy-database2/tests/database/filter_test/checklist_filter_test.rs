use flowy_database2::entities::{ChecklistFilterConditionPB, FieldType};
use flowy_database2::services::field::checklist_type_option::ChecklistCellData;

use crate::database::filter_test::script::FilterScript::*;
use crate::database::filter_test::script::{DatabaseFilterTest, FilterRowChanged};

#[tokio::test]
async fn grid_filter_checklist_is_incomplete_test() {
  let mut test = DatabaseFilterTest::new().await;
  let expected = 6;
  let row_count = test.row_details.len();
  let option_ids = get_checklist_cell_options(&test).await;

  let scripts = vec![
    UpdateChecklistCell {
      row_id: test.row_details[0].row.id.clone(),
      selected_option_ids: option_ids,
    },
    CreateChecklistFilter {
      condition: ChecklistFilterConditionPB::IsIncomplete,
      changed: Some(FilterRowChanged {
        showing_num_of_rows: 0,
        hiding_num_of_rows: row_count - expected,
      }),
    },
    AssertNumberOfVisibleRows { expected },
  ];
  test.run_scripts(scripts).await;
}

#[tokio::test]
async fn grid_filter_checklist_is_complete_test() {
  let mut test = DatabaseFilterTest::new().await;
  let expected = 1;
  let row_count = test.row_details.len();
  let option_ids = get_checklist_cell_options(&test).await;
  let scripts = vec![
    UpdateChecklistCell {
      row_id: test.row_details[0].row.id.clone(),
      selected_option_ids: option_ids,
    },
    CreateChecklistFilter {
      condition: ChecklistFilterConditionPB::IsComplete,
      changed: Some(FilterRowChanged {
        showing_num_of_rows: 0,
        hiding_num_of_rows: row_count - expected,
      }),
    },
    AssertNumberOfVisibleRows { expected },
  ];
  test.run_scripts(scripts).await;
}

async fn get_checklist_cell_options(test: &DatabaseFilterTest) -> Vec<String> {
  let field = test.get_first_field(FieldType::Checklist);
  let row_cell = test
    .editor
    .get_cell(&field.id, &test.row_details[0].row.id)
    .await;
  row_cell
    .map_or_else(
      || ChecklistCellData::default(),
      |cell| ChecklistCellData::from(&cell),
    )
    .options
    .into_iter()
    .map(|option| option.id)
    .collect()
}
