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
String excelFile01 = '/Users/jack.c/git/Katalon_API_Testing/Data Files/Login and Spin.xlsx'
workbook01 = ExcelKeywords.getWorkbook(excelFile01)
login = ExcelKeywords.getExcelSheet(workbook01, 'login')
take_turn = ExcelKeywords.getExcelSheet(workbook01, 'take_turn')

// Compare Response of login and take_turn
assert ExcelKeywords.getCellValueByIndex(login, j, 0) == ExcelKeywords.getCellValueByIndex(take_turn, j, 0)
assert ExcelKeywords.getCellValueByIndex(login, j, 1) == ExcelKeywords.getCellValueByIndex(take_turn, j, 1)
assert ExcelKeywords.getCellValueByIndex(login, j, 2) == ExcelKeywords.getCellValueByIndex(take_turn, j, 2)
assert ExcelKeywords.getCellValueByIndex(login, j, 3) == ExcelKeywords.getCellValueByIndex(take_turn, j, 3)
assert ExcelKeywords.getCellValueByIndex(login, j, 4) == ExcelKeywords.getCellValueByIndex(take_turn, j, 4)
assert ExcelKeywords.getCellValueByIndex(login, j, 5) == ExcelKeywords.getCellValueByIndex(take_turn, j, 5)
assert ExcelKeywords.getCellValueByIndex(login, j, 6) == ExcelKeywords.getCellValueByIndex(take_turn, j, 6)
assert ExcelKeywords.getCellValueByIndex(login, j, 7) == ExcelKeywords.getCellValueByIndex(take_turn, j, 7)
assert ExcelKeywords.getCellValueByIndex(login, j, 8) == ExcelKeywords.getCellValueByIndex(take_turn, j, 8)
assert ExcelKeywords.getCellValueByIndex(login, j, 9) == ExcelKeywords.getCellValueByIndex(take_turn, j, 9)
assert ExcelKeywords.getCellValueByIndex(login, j, 10) == ExcelKeywords.getCellValueByIndex(take_turn, j, 10)
assert ExcelKeywords.getCellValueByIndex(login, j, 11) == ExcelKeywords.getCellValueByIndex(take_turn, j, 11)
assert ExcelKeywords.getCellValueByIndex(login, j, 12) == ExcelKeywords.getCellValueByIndex(take_turn, j, 12)
assert ExcelKeywords.getCellValueByIndex(login, j, 13) == ExcelKeywords.getCellValueByIndex(take_turn, j, 13)
assert ExcelKeywords.getCellValueByIndex(login, j, 14) == ExcelKeywords.getCellValueByIndex(take_turn, j, 14)
assert ExcelKeywords.getCellValueByIndex(login, j, 15) == ExcelKeywords.getCellValueByIndex(take_turn, j, 15)
assert ExcelKeywords.getCellValueByIndex(login, j, 16) == ExcelKeywords.getCellValueByIndex(take_turn, j, 16)
assert ExcelKeywords.getCellValueByIndex(login, j, 17) == ExcelKeywords.getCellValueByIndex(take_turn, j, 17)
assert ExcelKeywords.getCellValueByIndex(login, j, 18) == ExcelKeywords.getCellValueByIndex(take_turn, j, 18)
assert ExcelKeywords.getCellValueByIndex(login, j, 19) == ExcelKeywords.getCellValueByIndex(take_turn, j, 19)
assert ExcelKeywords.getCellValueByIndex(login, j, 20) == ExcelKeywords.getCellValueByIndex(take_turn, j, 20)
assert ExcelKeywords.getCellValueByIndex(login, j, 21) == ExcelKeywords.getCellValueByIndex(take_turn, j, 21)
