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
import com.kms.katalon.keyword.excel.ExcelKeywords

import internal.GlobalVariable as GlobalVariable
import java.util.ArrayList as ArrayList
import jxl.write.WriteException as WriteException

get_session_token = WS.sendRequestAndVerify(findTestObject('Get_Session_Token'))

String session = GlobalVariable.session_token

println(session)

String excelFile01 = '/Users/jack.c/Katalon Studio/_TestData/Session_Token.xlsx'

// Create Excel File
ExcelKeywords.createExcelFile(excelFile01)

// Verify Excel File Be Created
File file01 = new File(excelFile01)
assert file01.exists() == true

// Create New Sheet
workbook01 = ExcelKeywords.getWorkbook(excelFile01)
ExcelKeywords.createExcelSheet(workbook01, 'Sheet1')
ExcelKeywords.saveWorkbook(excelFile01, workbook01)

// Verify Sheet Be Created
String[] ExpectedListSheetFile1 = ['Sheet0','Sheet1']
workbookFile01 = ExcelKeywords.getWorkbook(excelFile01) // get latest workbook File01
assert ExcelKeywords.getSheetNames(workbookFile01) == ExpectedListSheetFile1


// Write Data to Excel File
workbook01 = ExcelKeywords.getWorkbook(excelFile01)
sheet01 = ExcelKeywords.getExcelSheet(workbook01, 'Sheet1')
ExcelKeywords.setValueToCellByAddress(sheet01, 'A2', session)

// Save Data of Excel File
ExcelKeywords.saveWorkbook(excelFile01, workbook01)