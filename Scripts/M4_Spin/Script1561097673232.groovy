import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import java.lang.ProcessEnvironment.Variable as Variable
import org.eclipse.persistence.jpa.jpql.parser.SumFunction as SumFunction
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

WS.sendRequestAndVerify(findTestObject('Get_Session_Token', [('url') : 'krug-gw-colo.star9ad.com', ('partner') : 'c304afdf-2f61-6369-c088-924f99e1be1a'
            , ('secretkey') : '418184e911563cd861e90db6233d7d6c', ('player_id') : 'WalletPlayer05', ('session_token') : GlobalVariable.session_token]))

WS.sendRequestAndVerify(findTestObject('RGS/M4_Login', [('partner') : 'c304afdf-2f61-6369-c088-924f99e1be1a', ('session_token') : GlobalVariable.session_token
            , ('game_code') : 'M4-0008']))

WS.sendRequestAndVerify(findTestObject('RGS/M4_Init', [('partner') : 'c304afdf-2f61-6369-c088-924f99e1be1a', ('user_id') : GlobalVariable.user_id]))

String newline = System.getProperty('line.separator')

for (int i = 1; i <= 50; i++) {
    WS.sendRequestAndVerify(findTestObject('RGS/M4_Spin'))

    def win_loss = (GlobalVariable.total_win - 800) / 100

    println(('**** Win/Loss ****' + newline) + win_loss)
	
	
}

