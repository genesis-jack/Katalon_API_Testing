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
// Set PTH Data To Excel File
workbook01 = ExcelKeywords.getWorkbook(excelFile01)
pth = ExcelKeywords.getExcelSheet(workbook01, 'bo_pth_last_spin')
ExcelKeywords.setValueToCellByAddress(pth, 'A1', 'roundId')
ExcelKeywords.setValueToCellByIndex(pth, j, 0, GlobalVariable.pth_roundId)
ExcelKeywords.setValueToCellByAddress(pth, 'B1', 'balance')
ExcelKeywords.setValueToCellByIndex(pth, j, 1, GlobalVariable.pth_balance)
ExcelKeywords.setValueToCellByAddress(pth, 'C1', 'currency')
ExcelKeywords.setValueToCellByIndex(pth, j, 2, GlobalVariable.pth_currency)
ExcelKeywords.setValueToCellByAddress(pth, 'D1', 'txDate')
ExcelKeywords.setValueToCellByIndex(pth, j, 3, GlobalVariable.pth_txDate)
ExcelKeywords.setValueToCellByAddress(pth, 'E1', 'playerId')
ExcelKeywords.setValueToCellByIndex(pth, j, 4, GlobalVariable.pth_playerId)
ExcelKeywords.setValueToCellByAddress(pth, 'F1', 'gameId')
ExcelKeywords.setValueToCellByIndex(pth, j, 5, GlobalVariable.pth_gameId)
ExcelKeywords.setValueToCellByAddress(pth, 'G1', 'betPaid')
ExcelKeywords.setValueToCellByIndex(pth, j, 6, GlobalVariable.pth_betPaid)
ExcelKeywords.setValueToCellByAddress(pth, 'H1', 'betValue')
ExcelKeywords.setValueToCellByIndex(pth, j, 7, GlobalVariable.pth_betValue)
ExcelKeywords.setValueToCellByAddress(pth, 'I1', 'txId')
ExcelKeywords.setValueToCellByIndex(pth, j, 8, GlobalVariable.pth_txId)
ExcelKeywords.setValueToCellByAddress(pth, 'J1', 'win')
ExcelKeywords.setValueToCellByIndex(pth, j, 9, GlobalVariable.pth_win)
ExcelKeywords.setValueToCellByAddress(pth, 'K1', 'winLoss')
ExcelKeywords.setValueToCellByIndex(pth, j, 10, GlobalVariable.pth_winLoss)
ExcelKeywords.setValueToCellByAddress(pth, 'L1', 'providerId')
ExcelKeywords.setValueToCellByIndex(pth, j, 11, GlobalVariable.pth_providerId)
ExcelKeywords.setValueToCellByAddress(pth, 'M1', 'gameName')
ExcelKeywords.setValueToCellByIndex(pth, j, 12, GlobalVariable.pth_gameName)
ExcelKeywords.setValueToCellByAddress(pth, 'N1', 'subType')
ExcelKeywords.setValueToCellByIndex(pth, j, 13, GlobalVariable.pth_subType)
ExcelKeywords.setValueToCellByAddress(pth, 'O1', 'txTpye')
ExcelKeywords.setValueToCellByIndex(pth, j, 14, GlobalVariable.pth_txType)
ExcelKeywords.setValueToCellByAddress(pth, 'P1', 'device')
ExcelKeywords.setValueToCellByIndex(pth, j, 15, GlobalVariable.pth_device)
ExcelKeywords.setValueToCellByAddress(pth, 'Q1', 'timeStamp')
ExcelKeywords.setValueToCellByIndex(pth, j, 16, GlobalVariable.pth_timeStamp)
ExcelKeywords.setValueToCellByAddress(pth, 'R1', 'totalBetPaid')
ExcelKeywords.setValueToCellByIndex(pth, j, 17, GlobalVariable.pth_totalBetPaid)
ExcelKeywords.setValueToCellByAddress(pth, 'S1', 'totalWinLoss')
ExcelKeywords.setValueToCellByIndex(pth, j, 18, GlobalVariable.pth_totalWinLoss)
ExcelKeywords.setValueToCellByAddress(pth, 'T1', 'totalNum')
ExcelKeywords.setValueToCellByIndex(pth, j, 19, GlobalVariable.pth_totalNum)
ExcelKeywords.setValueToCellByAddress(pth, 'T1', 'totalBetValue')
ExcelKeywords.setValueToCellByIndex(pth, j, 20, GlobalVariable.pth_totalBetValue)
ExcelKeywords.setValueToCellByAddress(pth, 'V1', 'totalWin')
ExcelKeywords.setValueToCellByIndex(pth, j, 21, GlobalVariable.pth_totalWin)
// Save Data of Excel File
ExcelKeywords.saveWorkbook(excelFile01, workbook01)

// Set BO Transaction Data to Excel Fiel
workbook01 = ExcelKeywords.getWorkbook(excelFile01)
bo_transaction = ExcelKeywords.getExcelSheet(workbook01, 'bo_transaction')
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'A1', 'bonusproviderref')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 0, GlobalVariable.bo_bonusproviderref)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'B1', 'roundType')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 1, GlobalVariable.bo_roundType)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'C1', 'boosterType')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 2, GlobalVariable.bo_boosterType)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'D1', 'bonusBet')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 3, GlobalVariable.bo_bonusBet)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'E1', 'betValue')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 4, GlobalVariable.bo_betValue)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'F1', 'partner_data')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 5, GlobalVariable.bo_partner_data)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'G1', 'user_id')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 6, GlobalVariable.bo_user_id)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'H1', 'game_id')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 7, GlobalVariable.bo_game_id)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'I1', 'causality')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 8, GlobalVariable.bo_causality)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'J1', 'currency')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 9, GlobalVariable.bo_currency)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'K1', 'total_bet')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 10, GlobalVariable.bo_total_bet)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'L1', 'total_won')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 11, GlobalVariable.bo_total_won)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'M1', 'balance')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 12, GlobalVariable.bo_balance)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'N1', 'titmestamp')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 13, GlobalVariable.bo_timestamp)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'O1', 'merchantcode')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 14, GlobalVariable.bo_merchantcode)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'P1', 'device')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 15, GlobalVariable.bo_device)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'Q1', 'user_type')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 16, GlobalVariable.bo_user_type)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'R1', 'round_id')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 17, GlobalVariable.bo_round_id)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'S1', 'jp_id')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 18, GlobalVariable.bo_jp_id)
ExcelKeywords.setValueToCellByAddress(bo_transaction, 'T1', 'jpcontrib')
ExcelKeywords.setValueToCellByIndex(bo_transaction, j, 19, GlobalVariable.bo_jpcontrib)
// Save Data of Excel File
ExcelKeywords.saveWorkbook(excelFile01, workbook01)
