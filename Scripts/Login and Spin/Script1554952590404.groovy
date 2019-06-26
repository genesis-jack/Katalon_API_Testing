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

get_session_token = WS.sendRequestAndVerify(findTestObject('Wallet/Get_Session_Token', [('url') : 'krug-gw-colo.star9ad.com', ('partner') : Partner
			, ('secretkey') : Secret_Key, ('player_id') : Player_ID, ('session_token') : GlobalVariable.session_token]))

login = WS.sendRequestAndVerify(findTestObject('NuRGS/Login', [('session_token') : GlobalVariable.session_token, ('partner') : Partner
			, ('gamecode') : Game_Code]))

for (int i = 1; i <= 50; i++) {
	def features = GlobalVariable.features
	def features_type = GlobalVariable.features_type
	def free_spin_pick = GlobalVariable.free_spin_pick
	def free_spin_complete = GlobalVariable.free_spin_complete
	def free_spin_left = GlobalVariable.free_spin_left

// Free Spin Not Triggered
	if (features == null) {
		println('features = ' + features)
		spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_BaseSpin', [('player_id') : GlobalVariable.player_id
					, ('partner_code') : GlobalVariable.partner_code, ('rgs_session_token') : GlobalVariable.rgs_session_token
					, ('state_tag') : GlobalVariable.state_tag, ('gamecode') : Game_Code]))
	}
// Spin With Booster (eg. Free Run With Super Wild Booster)
	else if ((features != null) && !('PICK'.equals(features_type))) {
		println('features = ' + features)
		spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_BaseSpin', [('player_id') : GlobalVariable.player_id
					, ('partner_code') : GlobalVariable.partner_code, ('rgs_session_token') : GlobalVariable.rgs_session_token
					, ('state_tag') : GlobalVariable.state_tag, ('gamecode') : Game_Code]))

	}
// Free Spin Triggered And Not Pick
	else if ((features != null) && 'PICK'.equals(features_type)) {
		println('features = ' + features)
		println('free spin pick is = ' + free_spin_pick)

// Free Spin Triggered And Not Pick
		if (free_spin_pick != true) {
			WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_Pick', [('player_id') : GlobalVariable.player_id, ('partner_code') : GlobalVariable.partner_code
						, ('rgs_session_token') : GlobalVariable.rgs_session_token, ('state_tag') : GlobalVariable.state_tag
						, ('gamecode') : Game_Code]))
		}
		
// Free Spin Triggered And Pick
		else if (free_spin_pick == true) {
			println('features = ' + features)
			println('free spin pick is = ' + free_spin_pick)
			
// Free Spin Not Complete
			if (free_spin_complete != true) {
				WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_FreeSpin', [('player_id') : GlobalVariable.player_id, ('partner_code') : GlobalVariable.partner_code
							, ('rgs_session_token') : GlobalVariable.rgs_session_token, ('state_tag') : GlobalVariable.state_tag
							, ('gamecode') : Game_Code]))
				println('free spin left  = ' + free_spin_left)
			}
			
// Free Spin Complete
			else if (free_spin_complete == true) {
				println('features = ' + features)
				WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_BaseSpin', [('player_id') : GlobalVariable.player_id, ('partner_code') : GlobalVariable.partner_code
							, ('rgs_session_token') : GlobalVariable.rgs_session_token, ('state_tag') : GlobalVariable.state_tag
							, ('gamecode') : Game_Code]))
			}
		}
	}
}