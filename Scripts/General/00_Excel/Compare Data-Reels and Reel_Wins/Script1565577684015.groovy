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
rgstx = ExcelKeywords.getExcelSheet(workbook01, 'rgs_detail')
take_turn = ExcelKeywords.getExcelSheet(workbook01, 'take_turn')

// Compare reels symbo from API: rgs-history vs take_turn
assert ExcelKeywords.getCellValueByIndex(rgstx, j, 9) == ExcelKeywords.getCellValueByIndex(take_turn, j, 10)
// Compare reel_wins symbo from API: rgs-history vs take_turn
assert ExcelKeywords.getCellValueByIndex(rgstx, j, 10) == ExcelKeywords.getCellValueByIndex(take_turn, j, 11)