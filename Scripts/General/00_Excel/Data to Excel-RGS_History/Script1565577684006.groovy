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
// Set Detail History to Excel
workbook01 = ExcelKeywords.getWorkbook(excelFile01)
rgstx = ExcelKeywords.getExcelSheet(workbook01, 'rgs_detail')
ExcelKeywords.setValueToCellByAddress(rgstx, 'A1', 'round_id')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 0, GlobalVariable.rgs_round_id)
ExcelKeywords.setValueToCellByAddress(rgstx, 'B1', 'transaction_id')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 1, GlobalVariable.rgs_transaction_id)
ExcelKeywords.setValueToCellByAddress(rgstx, 'C1', 'transaction_time')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 2, GlobalVariable.rgs_transaction_time)
ExcelKeywords.setValueToCellByAddress(rgstx, 'D1', 'total_bet')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 3, GlobalVariable.rgs_total_bet)
ExcelKeywords.setValueToCellByAddress(rgstx, 'E1', 'bet_value')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 4, GlobalVariable.rgs_bet_value)
ExcelKeywords.setValueToCellByAddress(rgstx, 'F1', 'total_win')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 5, GlobalVariable.rgs_total_win)
ExcelKeywords.setValueToCellByAddress(rgstx, 'G1', 'balance')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 6, GlobalVariable.rgs_balance)
ExcelKeywords.setValueToCellByAddress(rgstx, 'H1', 'line')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 7, GlobalVariable.rgs_line)
ExcelKeywords.setValueToCellByAddress(rgstx, 'I1', 'reels')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 8, GlobalVariable.rgs_reels)
ExcelKeywords.setValueToCellByAddress(rgstx, 'J1', 'reels_wins')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 9, GlobalVariable.rgs_reel_wins)
ExcelKeywords.setValueToCellByAddress(rgstx, 'K1', 'scatter_win')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 10, GlobalVariable.rgs_scatter_win)
ExcelKeywords.setValueToCellByAddress(rgstx, 'L1', 'transaction_type')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 11, GlobalVariable.rgs_transaction_type)
ExcelKeywords.setValueToCellByAddress(rgstx, 'M1', 'features_triggered')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 12, GlobalVariable.rgs_features_triggered)
ExcelKeywords.setValueToCellByAddress(rgstx, 'N1', 'player_id')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 13, GlobalVariable.rgs_player_id)
ExcelKeywords.setValueToCellByAddress(rgstx, 'O1', 'currency')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 14, GlobalVariable.rgs_currency)
ExcelKeywords.setValueToCellByAddress(rgstx, 'P1', 'booster_type')
ExcelKeywords.setValueToCellByIndex(rgstx, j, 15, GlobalVariable.rgs_booster_type)
// Save Data of Excel File
ExcelKeywords.saveWorkbook(excelFile01, workbook01)
