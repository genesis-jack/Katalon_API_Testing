<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Buy Booster_Regular</name>
   <tag></tag>
   <elementGuidId>ecda6b91-88a7-45b6-8be3-50b2f07daf8e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;booster_id\&quot;: \&quot;FreeSpinBooster-001\&quot;,\n  \&quot;quantity\&quot;: 50,\n  \&quot;booster_type\&quot;: \&quot;REGULAR\&quot;,\n  \&quot;session_token\&quot;: \&quot;${rgs_session_token}\&quot;,\n  \&quot;action\&quot;: \&quot;BUY_BOOSTER\&quot;,\n  \&quot;game_code\&quot;: \&quot;NG-0063\&quot;,\n  \&quot;player_id\&quot;: \&quot;${player_id}\&quot;,\n  \&quot;partner_code\&quot;: \&quot;${partner_code}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-Genesis-PartnerToken</name>
      <type>Main</type>
      <value>${partner}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://nurgs.${env}.com/ng/take-turn/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>findTestData('GT_Wallet_Separation-GT_Genesis').getValue(1, 1)</defaultValue>
      <description></description>
      <id>6e9ff341-5f13-4398-8cdb-d34255d077f9</id>
      <masked>false</masked>
      <name>partner_code</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.rgs_session_token</defaultValue>
      <description></description>
      <id>cffe63cf-fdaa-4631-b2dd-f25710207923</id>
      <masked>false</masked>
      <name>rgs_session_token</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Test_Accounts_GT').getValue(3, 1)</defaultValue>
      <description></description>
      <id>07ca772a-0c8c-4f43-920d-7c8ed758c2b2</id>
      <masked>false</masked>
      <name>player_id</name>
   </variables>
   <variables>
      <defaultValue>'3655oule'</defaultValue>
      <description></description>
      <id>52443b16-e002-42d4-8b9d-63ebd81362ae</id>
      <masked>false</masked>
      <name>env</name>
   </variables>
   <variables>
      <defaultValue>findTestData('GT_Wallet_Separation-GT_Genesis').getValue(2, 1)</defaultValue>
      <description></description>
      <id>447ff5a8-7953-4c56-a0a1-acffc28b6b09</id>
      <masked>false</masked>
      <name>partner</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()




WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
