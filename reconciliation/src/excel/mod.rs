use crate::prelude::*;
use anyhow::Result;
use xlsxwriter::*;

pub struct Excel {
    workbook: Workbook,
}

macro_rules! write_headers {
    ($workbook:expr, $worksheet:expr) => {
        let header_format = $workbook
            .add_format()
            .set_bold()
            .set_font_size(16.0)
            .set_align(FormatAlignment::CenterAcross);
        $worksheet.write_string(0, 0, "数据源", Some(&header_format))?;
        for (index, field) in FlushData::fields().iter().enumerate() {
            $worksheet.write_string(0, (index + 1) as u16, field, Some(&header_format))?
        }
    };
}

macro_rules! write_one_result {
    ($sheet:expr, $result:expr, $row:expr, $af:expr, $cf:expr, $df:expr) => {
        $sheet.write_string($row, 0, &$result.name, None)?;
        $sheet.write_string($row, 1, &$result.data.tx_id, None)?;
        $sheet.write_string($row, 2, &$result.data.address, None)?;
        $sheet.write_string($row, 3, &$result.data.amount.to_string(), $af)?;
        $sheet.write_string($row, 4, &$result.data.currency, $cf)?;
        $sheet.write_string($row, 5, &$result.data.direction.to_string(), $df)?;
        $row += 1;
    };
}

macro_rules! write_one_blank {
    ($sheet:expr, $row:expr, $format:expr) => {
        for column in 0..6 {
            $sheet.write_blank($row, column, $format)?;
        }
        $row += 1;
    };
}

impl Excel {
    pub fn new<P: AsRef<str>>(path: P) -> Self {
        Excel {
            workbook: Workbook::new(path.as_ref()),
        }
    }

    pub fn write_sheet(&mut self, sheet: &str, data: &[StatementResult]) -> Result<()> {
        let mut worksheet = self.workbook.add_worksheet(Some(sheet))?;
        write_headers!(self.workbook, worksheet);

        let mut row = 1;
        let mismatch_format = self.workbook.add_format().set_bg_color(FormatColor::Gray);

        for data in data {
            match data {
                StatementResult::OneSide(one_result) => {
                    write_one_result!(worksheet, one_result, row, None, None, None);
                    write_one_blank!(worksheet, row, Some(&mismatch_format));
                }
                StatementResult::DataMismatch(results, mismatches) => {
                    let mut amount_format = None;
                    let mut currency_format = None;
                    let mut direction_format = None;
                    for result in results {
                        for mismatch in mismatches {
                            match mismatch {
                                FlushDataMismatch::Amount => {
                                    amount_format = Some(&mismatch_format);
                                }
                                FlushDataMismatch::Currency => {
                                    currency_format = Some(&mismatch_format);
                                }
                                FlushDataMismatch::Direction => {
                                    direction_format = Some(&mismatch_format);
                                }
                            }
                        }
                        write_one_result!(
                            worksheet,
                            result,
                            row,
                            amount_format,
                            currency_format,
                            direction_format
                        );
                    }
                }
            }
            row += 1;
        }
        Ok(())
    }

    pub fn save(self) -> Result<()> {
        self.workbook.close().map_err(Into::into)
    }
}
