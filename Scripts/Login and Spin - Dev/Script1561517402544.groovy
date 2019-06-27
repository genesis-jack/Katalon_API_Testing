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

WS.sendRequestAndVerify(findTestObject('Wallet/Get_Session_Token', [('url') : 'krug-gw-colo.star9ad.com', ('partner') : Partner
			, ('secretkey') : Secret_Key, ('player_id') : Player_ID, ('session_token') : GlobalVariable.session_token]))

WS.sendRequestAndVerify(findTestObject('NuRGS/Login', [('session_token') : GlobalVariable.session_token, ('partner') : Partner
			, ('gamecode') : Game_Code]))

for (int i = 1; i <= 10; i++) {
	def features = GlobalVariable.features
	def features_type = GlobalVariable.features_type
	def free_spin_pick = GlobalVariable.free_spin_pick
	def free_spin_complete = GlobalVariable.free_spin_complete
	def free_spin_left = GlobalVariable.free_spin_left

	if (features == null) {
		// Feature Not Triggered
		println('features = ' + features)
		println('Complete = ' + free_spin_complete)
		spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_BaseSpin', [('player_id') : GlobalVariable.player_id
					, ('partner_code') : GlobalVariable.partner_code, ('rgs_session_token') : GlobalVariable.rgs_session_token
					, ('state_tag') : GlobalVariable.state_tag, ('gamecode') : Game_Code]))
	}
	else if ((features != null) && ('FREE_SPIN'.equals(features_type) && (free_spin_complete != true))) {
		// Feature Triggered And Type Is "FREE_SPIN" And Not Complete
		println('features = ' + features)
		println('Complete = ' + free_spin_complete)
		spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_FreeSpin', [('player_id') : GlobalVariable.player_id, ('partner_code') : GlobalVariable.partner_code
					, ('rgs_session_token') : GlobalVariable.rgs_session_token, ('state_tag') : GlobalVariable.state_tag
					, ('gamecode') : Game_Code]))
		println('free spin left  = ' + free_spin_left)
	}
	else if ((features != null) && ('FREE_SPIN'.equals(features_type) && (free_spin_complete == true))) {
		// Feature Triggered And Type Is "FREE_SPIN" And Complete
		println('features = ' + features)
		spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_BaseSpin', [('player_id') : GlobalVariable.player_id, ('partner_code') : GlobalVariable.partner_code
					, ('rgs_session_token') : GlobalVariable.rgs_session_token, ('state_tag') : GlobalVariable.state_tag
					, ('gamecode') : Game_Code]))
		println('free spin left  = ' + free_spin_left)
	}	
	else if ((features != null) && 'PICK'.equals(features_type)) {
		// Feature Triggered And Type Is "PICK"
		println('features = ' + features)
		println('free spin pick is = ' + free_spin_pick)
		if (free_spin_pick != true) {
			// Not Pick Yet
			spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_Pick', [('player_id') : GlobalVariable.player_id, ('partner_code') : GlobalVariable.partner_code
						, ('rgs_session_token') : GlobalVariable.rgs_session_token, ('state_tag') : GlobalVariable.state_tag
						, ('gamecode') : Game_Code]))
		}
		else if (free_spin_pick == true) {
			// Picked
			println('features = ' + features)
			println('free spin pick is = ' + free_spin_pick)
			if (free_spin_complete != true) {
				// Free Spin Not Complete
				spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_FreeSpin', [('player_id') : GlobalVariable.player_id, ('partner_code') : GlobalVariable.partner_code
							, ('rgs_session_token') : GlobalVariable.rgs_session_token, ('state_tag') : GlobalVariable.state_tag
							, ('gamecode') : Game_Code]))
				println('free spin left  = ' + free_spin_left)
			}			
			else if (free_spin_complete == true) {
				// Free Spin Complete
				println('features = ' + features)
				spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_BaseSpin', [('player_id') : GlobalVariable.player_id, ('partner_code') : GlobalVariable.partner_code
							, ('rgs_session_token') : GlobalVariable.rgs_session_token, ('state_tag') : GlobalVariable.state_tag
							, ('gamecode') : Game_Code]))
			}
		}
	}
}