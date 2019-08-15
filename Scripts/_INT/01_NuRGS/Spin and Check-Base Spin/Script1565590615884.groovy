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
import net.bytebuddy.implementation.bytecode.constant.NullConstant as NullConstant
import java.util.ArrayList as ArrayList
import java.io.File as File
import com.kms.katalon.keyword.excel.ExcelKeywords as ExcelKeywords

String excelFile01 = '/Users/jack.c/git/Katalon_API_Testing/Data Files/INT_Login and BaseSpin.xlsx'

// Create excel files
ExcelKeywords.createExcelFile(excelFile01)

// Verify files are created
File file1 = new File(excelFile01)

assert file1.exists() == true

// Create list of sheets with name
workbook01 = ExcelKeywords.getWorkbook(excelFile01)

ExcelKeywords.createExcelSheets(workbook01, ['login', 'take_turn', 'rgs_detail', 'bo_pth_last_spin', 'bo_transaction'])

ExcelKeywords.saveWorkbook(excelFile01, workbook01)

WS.sendRequestAndVerify(findTestObject('Wallet/Get_Session_Token', [('env') : Env_krug_gw, ('partner') : Partner
			, ('secretkey') : Secret_Key, ('player_id') : Player_ID]))

WS.sendRequestAndVerify(findTestObject('NuRGS/Login', [('env') : Env_RGS, ('session_token') : GlobalVariable.session_token, ('partner') : Partner
			, ('game_code') : Game_Code]))

for (int i = 0; i <= 1; i++) {
	def features = GlobalVariable.features

	def features_type = GlobalVariable.features_type

	def free_spin_pick = GlobalVariable.free_spin_pick

	def free_spin_complete = GlobalVariable.free_spin_complete

	def free_spin_left = GlobalVariable.free_spin_left

	String newline = System.getProperty('line.separator')

	if (features == null) {
		// Feature Not Triggered
		println(('**** In Test Case - Feature is ****' + newline) + features)
		println(('**** In Test Case - Complete = **** ' + newline) + free_spin_complete)
		spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_BaseSpin', [('env') : Env_RGS, ('player_id') : GlobalVariable.player_id
					, ('partner_code') : GlobalVariable.partner_code, ('rgs_session_token') : GlobalVariable.rgs_session_token
					, ('state_tag') : GlobalVariable.state_tag, ('game_code') : Game_Code]))
		break;
	} else if ((features != null) && ('FREE_SPIN'.equals(features_type) && (free_spin_complete != true))) {
		// Feature Triggered And Type Is "FREE_SPIN" And Not Complete
		println(('**** In Test Case - Feature is ****' + newline) + features)
		println(('**** In Test Case - Complete = **** ' + newline) + free_spin_complete)
		spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_FreeSpin', [('env') : Env_RGS, ('player_id') : GlobalVariable.player_id
					, ('partner_code') : GlobalVariable.partner_code, ('rgs_session_token') : GlobalVariable.rgs_session_token
					, ('state_tag') : GlobalVariable.state_tag, ('game_code') : Game_Code]))
		println(('**** In Test Case - Free Spin Left **** = ' + newline) + GlobalVariable.free_spin_left)
	} else if ((features != null) && ('FREE_SPIN'.equals(features_type) && (free_spin_complete == true))) {
		// Feature Triggered And Type Is "FREE_SPIN" And Complete
		println(('**** In Test Case - Feature is ****' + newline) + features)
		spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_BaseSpin', [('env') : Env_RGS, ('player_id') : GlobalVariable.player_id
					, ('partner_code') : GlobalVariable.partner_code, ('rgs_session_token') : GlobalVariable.rgs_session_token
					, ('state_tag') : GlobalVariable.state_tag, ('game_code') : Game_Code]))
		println(('**** In Test Case - Free Spin Left **** = ' + newline) + GlobalVariable.free_spin_left)
	} else if ((features != null) && 'PICK'.equals(features_type)) {
		// Feature Triggered And Type Is "PICK"
		println(('**** In Test Case - Feature is ****' + newline) + features)
		println(('**** In Test Case - Free Spin Pick is ****' + newline) + free_spin_pick)
		if (free_spin_pick != true) {
			// Not Pick Yet
			spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_Pick', [('env') : Env_RGS, ('player_id') : GlobalVariable.player_id
						, ('partner_code') : GlobalVariable.partner_code, ('rgs_session_token') : GlobalVariable.rgs_session_token
						, ('state_tag') : GlobalVariable.state_tag, ('game_code') : Game_Code]))
		} else if (free_spin_pick == true) {
			// Picked
			println(('**** In Test Case - Feature is ****' + newline) + features)
			println(('**** In Test Case - Free Spin Pick is ****' + newline) + free_spin_pick)
			if (free_spin_complete != true) {
				// Free Spin Not Complete
				spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_FreeSpin', [('env') : Env_RGS, ('player_id') : GlobalVariable.player_id
							, ('partner_code') : GlobalVariable.partner_code, ('rgs_session_token') : GlobalVariable.rgs_session_token
							, ('state_tag') : GlobalVariable.state_tag, ('game_code') : Game_Code]))

				println(('**** In Test Case - Free Spin Left is ****' + newline) + GlobalVariable.free_spin_left)
			} else if (free_spin_complete == true) {
				// Free Spin Complete
				println(('**** In Test Case - Feature is ****' + newline) + features)
				spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_BaseSpin', [('env') : Env_RGS, ('player_id') : GlobalVariable.player_id
							, ('partner_code') : GlobalVariable.partner_code, ('rgs_session_token') : GlobalVariable.rgs_session_token
							, ('state_tag') : GlobalVariable.state_tag, ('game_code') : Game_Code]))
			}
		}
	}
	
	WS.sendRequestAndVerify(findTestObject('Wallet/Get_Session_Token', [('env') : Env_krug_gw, ('partner') : Partner
				, ('secretkey') : Secret_Key, ('player_id') : Player_ID]))

	WS.sendRequestAndVerify(findTestObject('NuRGS/Login', [('env') : Env_RGS, ('session_token') : GlobalVariable.session_token, ('partner') : Partner
				, ('game_code') : Game_Code]))

	GlobalVariable.excel_col = (i + 1)
	// Write Login and Spin Data to Excel
	WebUI.callTestCase(findTestCase('General/00_Excel/Data to Excel-Login and Spin'), [:], FailureHandling.CONTINUE_ON_FAILURE)
	// Compare Login and Spin Data
	WebUI.callTestCase(findTestCase('General/00_Excel/Compare Data-Login vs Spin'), [:], FailureHandling.CONTINUE_ON_FAILURE)
	// Query Spin Data By rgs-history API
	WebUI.callTestCase(findTestCase('_INT/01_NuRGS/RGS History-Base Spin'), [:], FailureHandling.CONTINUE_ON_FAILURE)
	// Write rgs-history Response To Excel
	WebUI.callTestCase(findTestCase('General/00_Excel/Data to Excel-RGS_History'), [:], FailureHandling.CONTINUE_ON_FAILURE)
	// Query Spin Data By PTH API
	WebUI.callTestCase(findTestCase('_INT/02_Back Office/PTH-Detail-Base Spin'), [:], FailureHandling.CONTINUE_ON_FAILURE)
	// Write PTH API Response To Excel
	WebUI.callTestCase(findTestCase('General/00_Excel/Data to Excel-PTH'), [:], FailureHandling.CONTINUE_ON_FAILURE)
	// Compare PTH and rgs-history
	WebUI.callTestCase(findTestCase('General/00_Excel/Compare Data-PTH vs RGS_History'), [:], FailureHandling.CONTINUE_ON_FAILURE)
	// Compare Reels and Reel_Wins By take_turn and rgs-history API
	WebUI.callTestCase(findTestCase('General/00_Excel/Compare Data-PTH vs RGS_History'), [:], FailureHandling.CONTINUE_ON_FAILURE)
}