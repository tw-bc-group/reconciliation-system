use crate::prelude::*;
use anyhow::Result;
use chrono::Duration;
use xlsxwriter::*;

pub struct Excel {
    workbook: Workbook,
}

impl From<TransactionTime> for DateTime {
    fn from(d: TransactionTime) -> DateTime {
        let d = d + Duration::hours(8);
        DateTime::new(
            d.year() as i16,
            d.month() as i8,
            d.day() as i8,
            d.hour() as i8,
            d.minute() as i8,
            d.second() as f64,
        )
    }
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
    ($sheet:expr, $result:expr, $row:expr, $one:expr, $af:expr, $cf:expr, $df:expr, $tf:expr) => {
        $sheet.write_string($row, 0, &$result.name, $one)?;
        $sheet.write_string($row, 1, &$result.data.tx_id, None)?;
        $sheet.write_string($row, 2, &$result.data.address, None)?;
        $sheet.write_string($row, 3, &$result.data.amount.to_string(), $af)?;
        $sheet.write_string($row, 4, &$result.data.currency, $cf)?;
        $sheet.write_string($row, 5, &$result.data.direction.to_string(), $df)?;
        $sheet.write_datetime($row, 6, &$result.data.datetime.into(), $tf)?;
        $row += 1;
    };
}

macro_rules! write_one_blank {
    ($sheet:expr, $row:expr, $format:expr) => {
        for column in 0..7 {
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
        let mut time_format = self
            .workbook
            .add_format()
            .set_num_format("dd/mm/yyyy hh:mm:ss");
        let mismatch_format = self.workbook.add_format().set_bg_color(FormatColor::Gray);

        for data in data {
            match data {
                StatementResult::OneSide(one_result) => {
                    time_format = time_format.set_bg_color(FormatColor::White);
                    write_one_result!(
                        worksheet,
                        one_result,
                        row,
                        Some(&mismatch_format),
                        None,
                        None,
                        None,
                        Some(&time_format)
                    );
                    write_one_blank!(worksheet, row, Some(&mismatch_format));
                }
                StatementResult::DataMismatch(results, mismatches) => {
                    let (mut amount_format, mut currency_format, mut direction_format) =
                        (None, None, None);

                    for result in results {
                        time_format = time_format.set_bg_color(FormatColor::White);
                        for mismatch in mismatches {
                            match mismatch {
                                FlushDataMismatch::Amount => {
                                    amount_format = Some(&mismatch_format);
                                }
                                FlushDataMismatch::CrossDate => {
                                    time_format = time_format.set_bg_color(FormatColor::Gray);
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
                            None,
                            amount_format,
                            currency_format,
                            direction_format,
                            Some(&time_format)
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
