import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.keyword.excel.ExcelKeywords as ExcelKeywords

int j = GlobalVariable.excel_col
String excelFile01 = GlobalVariable.excel_path
workbook01 = ExcelKeywords.getWorkbook(excelFile01)
pth = ExcelKeywords.getExcelSheet(workbook01, 'bo_pth_last_spin')
rgstx = ExcelKeywords.getExcelSheet(workbook01, 'rgs_detail')

// Compare Detail History: PTH API vs. RGS History
// Compare Balance
assert ExcelKeywords.getCellValueByIndex(pth, j, 1) * 100 == ExcelKeywords.getCellValueByIndex(rgstx, j, 6) // Because of PTH API set this field as "String"
// Compare Round ID
assert ExcelKeywords.getCellValueByIndex(pth, j, 0) == ExcelKeywords.getCellValueByIndex(rgstx, j, 0)
// Compare Transaction ID
assert ExcelKeywords.getCellValueByIndex(pth, j, 8) == ExcelKeywords.getCellValueByIndex(rgstx, j, 1)
// Compare Currency
assert ExcelKeywords.getCellValueByIndex(pth, j, 2) == ExcelKeywords.getCellValueByIndex(rgstx, j, 14)
// Compare Player ID
assert ExcelKeywords.getCellValueByIndex(pth, j, 4) == ExcelKeywords.getCellValueByIndex(rgstx, j, 13)
// Compare Bet Value
assert ExcelKeywords.getCellValueByIndex(pth, j, 7) * 100 == ExcelKeywords.getCellValueByIndex(rgstx, j, 4) // Because of PTH API set this field as "String"
// Compare Bet Paid
assert ExcelKeywords.getCellValueByIndex(pth, j, 6) * 100 == ExcelKeywords.getCellValueByIndex(rgstx, j, 3) // Because of PTH API set this field as "String"
// Compare Total Win
assert ExcelKeywords.getCellValueByIndex(pth, j, 9) * 100 == ExcelKeywords.getCellValueByIndex(rgstx, j, 5) // Because of PTH API set this field as "String"