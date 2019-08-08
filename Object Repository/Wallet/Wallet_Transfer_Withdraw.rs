<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Wallet_Transfer_Withdraw</name>
   <tag></tag>
   <elementGuidId>7c40eddc-3f14-482e-b0a8-e8d138af808c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;user_id\&quot;: \&quot;${player_id}\&quot;,\n  \&quot;partner_id\&quot;: \&quot;${partner}\&quot;,\n  \&quot;credits\&quot;: 50000000000,\n  \&quot;currency\&quot;: \&quot;${currency}\&quot;,\n  \&quot;custom_json\&quot;: null,\n  \&quot;action\&quot;: \&quot;Withdraw\&quot;,\n  \&quot;external_transaction_id\&quot;: \&quot;\&quot;\n}&quot;,
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
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-Genesis-Secret</name>
      <type>Main</type>
      <value>${secretkey}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://${env}:8088/m4/wallet/transfer</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>findTestData('Environment').getValue(2, 2)</defaultValue>
      <description></description>
      <id>cb79b9f4-bb0b-49c5-83fa-fda13ac378f9</id>
      <masked>false</masked>
      <name>env</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Test_Accounts_GT').getValue(3, 1)</defaultValue>
      <description></description>
      <id>256ae22d-941b-4648-9de1-8de0c4007509</id>
      <masked>false</masked>
      <name>player_id</name>
   </variables>
   <variables>
      <defaultValue>'CNY'</defaultValue>
      <description></description>
      <id>9ec099b7-3ad7-4ad4-99c0-c86712192282</id>
      <masked>false</masked>
      <name>currency</name>
   </variables>
   <variables>
      <defaultValue>'dcd887e5-5069-45d1-8a5a-b9af9b646f15'</defaultValue>
      <description></description>
      <id>17d03577-233d-49b9-983b-cab3976785a4</id>
      <masked>false</masked>
      <name>partner</name>
   </variables>
   <variables>
      <defaultValue>'1461871a-3e65-4edb-9f88-0e267948ed39'</defaultValue>
      <description></description>
      <id>fa91f9ea-10b0-4bb3-8274-6954d1e0cba3</id>
      <masked>false</masked>
      <name>secretkey</name>
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
