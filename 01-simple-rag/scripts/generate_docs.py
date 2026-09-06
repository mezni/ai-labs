import os
from weasyprint import HTML

output_dir = "data/policies"
os.makedirs(output_dir, exist_ok=True)

CSS_STYLE = """
@page {
    size: A4;
    margin: 18mm 15mm 18mm 15mm;
    background-color: #fafbfc;
    @bottom-right {
        content: "Page " counter(page) " of " counter(pages);
        font-family: 'Helvetica Neue', Arial, sans-serif;
        font-size: 8pt;
        color: #718096;
    }
    @bottom-left {
        content: "Aether Wireless • Corporate Compliance & Policies";
        font-family: 'Helvetica Neue', Arial, sans-serif;
        font-size: 8pt;
        color: #718096;
    }
}

*, *::before, *::after { box-sizing: border-box; }

body {
    font-family: 'Helvetica Neue', Arial, sans-serif;
    margin: 0; padding: 0;
    color: #2d3748; font-size: 9.5pt; line-height: 1.5;
    background-color: #fafbfc;
}

.header-banner {
    background: linear-gradient(135deg, #0f172a 0%, #1e293b 60%, #0284c7 100%);
    color: #ffffff;
    margin: -18mm -15mm 20px -15mm;
    padding: 22px 20mm 18px 20mm;
    border-bottom: 4px solid #38bdf8;
}

.company-logo {
    font-size: 18pt; font-weight: 800; letter-spacing: 1.5px;
    text-transform: uppercase; color: #ffffff; margin: 0 0 4px 0;
}
.company-logo span { color: #38bdf8; }

.document-title {
    font-size: 12pt; font-weight: 300; color: #94a3b8; margin: 0;
}

.meta-grid {
    display: table; width: 100%; margin-bottom: 20px;
    background-color: #ffffff; border: 1px solid #e2e8f0;
    border-radius: 6px; padding: 10px 15px;
}
.meta-row { display: table-row; }
.meta-cell { display: table-cell; padding: 4px 10px; font-size: 8.5pt; }
.meta-label { font-weight: bold; color: #475569; width: 20%; }
.meta-value { color: #0f172a; width: 30%; }

h2 {
    font-size: 11.5pt; font-weight: 700; color: #0f172a;
    border-left: 4px solid #0284c7; padding-left: 10px;
    margin-top: 20px; margin-bottom: 8px; page-break-after: avoid;
}

h3 {
    font-size: 10pt; font-weight: 600; color: #1e293b;
    margin-top: 14px; margin-bottom: 6px; page-break-after: avoid;
}

p, ul, ol { margin-top: 0; margin-bottom: 10px; }
p { text-align: justify; }
ul, ol { padding-left: 20px; }
li { margin-bottom: 4px; text-align: justify; }

.callout {
    background-color: #f0f9ff; border: 1px solid #bae6fd;
    border-left: 4px solid #0284c7; border-radius: 4px;
    padding: 10px 14px; margin: 12px 0; font-size: 9pt;
    page-break-inside: avoid;
}
.callout-title { font-weight: bold; color: #0369a1; margin-bottom: 4px; }

table {
    width: 100%; border-collapse: collapse; margin: 14px 0;
    font-size: 8.5pt; page-break-inside: avoid;
}
th {
    background-color: #0f172a; color: #ffffff; font-weight: 600;
    text-align: left; padding: 8px 10px; border: 1px solid #0f172a;
}
td {
    padding: 7px 10px; border: 1px solid #e2e8f0; background: #ffffff;
}
tr:nth-child(even) td { background-color: #f8fafc; }

.signature-block {
    margin-top: 30px; border-top: 1px solid #cbd5e1;
    padding-top: 15px; page-break-inside: avoid;
}
.sig-grid { display: table; width: 100%; }
.sig-col { display: table-cell; width: 50%; padding-right: 20px; }
.sig-line { border-bottom: 1px solid #94a3b8; height: 35px; margin-bottom: 5px; }
.sig-title { font-size: 8pt; color: #64748b; text-transform: uppercase; letter-spacing: 0.5px; }
"""

policies_data = [
    {
        "filename": "acceptable_use_policy.pdf",
        "doc_id": "POL-AETH-2026-V1",
        "title": "Terms of Service & Network Acceptable Use Policy (AUP)",
        "body": """
            <h2>1. Overview and Intent</h2>
            <p>Aether Wireless ("Company", "we", "us", or "our") is committed to delivering high-speed, reliable, and low-latency cloud-native 5G/6G connectivity to all subscribers. This Acceptable Use Policy ("AUP") defines the rules and guidelines governing the use of Aether Wireless's voice, data, text, and cloud network services (collectively, the "Services").</p>
            <p>By accessing or using the Services, subscribers ("Users" or "Customers") agree to strictly adhere to the terms set forth in this document.</p>

            <h2>2. Fair Usage Policy (FUP) & Bandwidth Governance</h2>
            <p>To preserve network quality and avoid congestion for all subscribers, Aether Wireless employs dynamic network management practices across unlimited and tiered data plans.</p>
            <table>
                <thead>
                    <tr>
                        <th>Service Plan Tier</th>
                        <th>Priority Data Threshold</th>
                        <th>Post-Threshold Management</th>
                        <th>Included Roaming</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td><strong>Aether Essential</strong></td>
                        <td>50 GB / month</td>
                        <td>Throttled to 1 Mbps during network congestion</td>
                        <td>5 GB Domestic / 0 GB Intl</td>
                    </tr>
                    <tr>
                        <td><strong>Aether Premier 5G</strong></td>
                        <td>150 GB / month</td>
                        <td>Dynamic QoS adjustments; minimum 10 Mbps guarantee</td>
                        <td>25 GB Domestic / 5 GB Intl</td>
                    </tr>
                    <tr>
                        <td><strong>Aether Enterprise Pro</strong></td>
                        <td>Uncapped Priority</td>
                        <td>Dedicated network slicing with guaranteed SLA</td>
                        <td>Uncapped Domestic / 25 GB Intl</td>
                    </tr>
                </tbody>
            </table>
            <div class="callout">
                <div class="callout-title">Notice on Automated Traffic Management:</div>
                Aether Wireless utilizes automated Deep Packet Inspection (DPI) solely for traffic prioritization and routing optimization. Content payload data is never inspected or logged.
            </div>

            <h2>3. Prohibited and Unlawful Uses</h2>
            <p>Subscribers are strictly prohibited from utilizing Aether Wireless infrastructure for any activities deemed unlawful, abusive, or disruptive to network operations:</p>
            <ul>
                <li><strong>Network Attacks & Tampering:</strong> Launching Denial of Service (DoS/DDoS) attacks, distributing malware, or unauthorized scanning.</li>
                <li><strong>Spam and Unsolicited Messaging:</strong> Automated mass-messaging or telephone spam schemes in violation of privacy laws.</li>
                <li><strong>Unauthorized Resale:</strong> Reselling network bandwidth or establishing unauthorized public hotspots.</li>
                <li><strong>Illegal Content Transmission:</strong> Storing or transmitting material that violates intellectual property or criminal regulations.</li>
            </ul>

            <h2>4. Network Security & Device Standards</h2>
            <p>Customers are responsible for ensuring that all devices connected to the Aether Wireless network meet official regulatory standards (e.g., FCC, CE) and are free from malicious software.</p>

            <h2>5. Policy Enforcement & Remedies</h2>
            <p>Failure to comply with this policy may result in progressive enforcement actions:</p>
            <ol>
                <li><strong>Warning Notice:</strong> Digital notification requesting immediate mitigation.</li>
                <li><strong>Temporary Restriction:</strong> Protocol restriction or dynamic QoS throttling.</li>
                <li><strong>Service Suspension:</strong> Temporary suspension of voice and data service access.</li>
                <li><strong>Permanent Termination:</strong> Account closure and forfeiture of remaining prepaid credits.</li>
            </ol>
        """
    },
    {
        "filename": "billing_invoice_policy.pdf",
        "doc_id": "POL-AETH-BILL-001",
        "title": "Billing & Invoice Policy",
        "body": """
            <h2>1. Billing Cycle & Statement Issuance</h2>
            <p>Aether Wireless issues monthly electronic statements on the 1st of each calendar month. Statements are made available via the subscriber's online account portal and transmitted to the registered email address on file.</p>

            <h2>2. Payment Due Dates & Late Fees</h2>
            <ul>
                <li><strong>Due Date:</strong> Payments are due exactly 21 days from the statement date.</li>
                <li><strong>Late Grace Period:</strong> A 5-day grace period is granted following the due date. Accounts not settled within this window are subject to a $5.00 late fee.</li>
                <li><strong>Service Interruption:</strong> Accounts overdue by more than 30 days may be subject to service suspension.</li>
            </ul>

            <h2>3. Disputed Charges</h2>
            <p>Subscribers must submit billing disputes within 60 days of the statement date. Disputes should be directed to billing@aetherwireless.com with the subject line "Billing Dispute - [Account Number]".</p>

            <h2>4. Refund Policy</h2>
            <p>Refunds for overpayments or early service termination are processed within 10 business days and applied as a credit to the subscriber's account or returned via the original payment method.</p>
        """
    },
    {
        "filename": "privacy_data_policy.pdf",
        "doc_id": "POL-AETH-PRIV-001",
        "title": "Privacy & Data Protection Policy",
        "body": """
            <h2>1. Data Collection & Usage</h2>
            <p>Aether Wireless collects personal and usage data solely for the purpose of delivering and improving our services. We adhere to strict data minimization principles in compliance with applicable privacy regulations including GDPR and CCPA.</p>

            <h2>2. Data Sharing & Third Parties</h2>
            <p>We do not sell subscriber data to third parties. Data may be shared with trusted service providers solely for network operations, billing, and customer support, under strict data processing agreements.</p>

            <h2>3. Data Retention</h2>
            <p>Subscriber data is retained only for the duration of the service relationship and for a period of 12 months following account closure, unless longer retention is required by law.</p>

            <h2>4. Subscriber Rights</h2>
            <p>Subscribers have the right to access, correct, port, and request deletion of their personal data. Requests should be directed to privacy@aetherwireless.com.</p>

            <h2>5. Security Measures</h2>
            <p>Aether Wireless employs industry-standard encryption (AES-256), multi-factor authentication, and continuous security monitoring to protect subscriber data against unauthorized access.</p>
        """
    },
    {
        "filename": "network_service_policy.pdf",
        "doc_id": "POL-AETH-NET-001",
        "title": "Network Service Level Agreement (SLA)",
        "body": """
            <h2>1. Service Commitments</h2>
            <p>Aether Wireless guarantees the following minimum service levels for enterprise subscribers:</p>
            <table>
                <thead>
                    <tr>
                        <th>Metric</th>
                        <th>Target</th>
                        <th>Measurement Period</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td><strong>Network Uptime</strong></td>
                        <td>99.95%</td>
                        <td>Monthly</td>
                    </tr>
                    <tr>
                        <td><strong>Average Latency</strong></td>
                        <td>&lt; 15ms</td>
                        <td>Monthly</td>
                    </tr>
                    <tr>
                        <td><strong>Download Speed</strong></td>
                        <td>&ge; 500 Mbps (Premier)</td>
                        <td>Monthly</td>
                    </tr>
                    <tr>
                        <td><strong>Incident Response</strong></td>
                        <td>&lt; 1 hour</td>
                        <td>Per incident</td>
                    </tr>
                </tbody>
            </table>

            <h2>2. Service Credits</h2>
            <p>If Aether Wireless fails to meet the guaranteed SLA metrics, subscribers are eligible for service credits:</p>
            <ul>
                <li>Uptime below 99.95% but above 99.0%: 5% monthly credit</li>
                <li>Uptime below 99.0%: 15% monthly credit</li>
                <li>Uptime below 95.0%: 30% monthly credit + escalation to executive review</li>
            </ul>

            <h2>3. Scheduled Maintenance</h2>
            <p>Planned maintenance windows are communicated at least 72 hours in advance and are scheduled during off-peak hours (2:00 AM - 5:00 AM local time) to minimize subscriber impact.</p>

            <h2>4. Exclusions</h2>
            <p>SLA guarantees do not apply to disruptions caused by force majeure events, subscriber equipment failure, or third-party infrastructure beyond Aether Wireless's control.</p>
        """
    },
    {
        "filename": "data_plan_terms_conditions.pdf",
        "doc_id": "POL-AETH-DATA-001",
        "title": "Data Plan Terms & Conditions",
        "body": """
            <h2>1. Plan Overview & Data Allowances</h2>
            <p>Each Aether Wireless data plan includes a specific monthly data allowance as outlined in the subscriber's service agreement. Unused data does not roll over unless explicitly stated in the plan tier.</p>

            <h2>2. Data Speed & Throttling</h2>
            <p>Upon exceeding the monthly high-speed data allowance, data speeds may be reduced to a lower priority tier subject to network availability. Fair Usage Policy (FUP) thresholds apply as defined in the Acceptable Use Policy.</p>

            <h2>3. International Roaming</h2>
            <p>Data roaming allowances vary by plan and destination country. Roaming rates and allowances are published in the rate card and are subject to change with 30 days' notice.</p>

            <h2>4. Plan Changes</h2>
            <p>Subscribers may change their data plan at any time. Plan changes take effect on the next billing cycle. A pro-rated adjustment may apply for mid-cycle upgrades.</p>

            <h2>5. Usage Monitoring</h2>
            <p>Subscribers can monitor data usage through the mobile app or online portal. Usage alerts are sent at 50%, 80%, and 100% of the monthly allowance.</p>
        """
    },
    {
        "filename": "device_protection_policy.pdf",
        "doc_id": "POL-AETH-DEV-001",
        "title": "Device Protection Policy",
        "body": """
            <h2>1. Coverage Overview</h2>
            <p>Aether Wireless Device Protection provides coverage against accidental damage, malfunctions, theft, and loss for eligible devices enrolled in the program.</p>

            <h2>2. Covered Events</h2>
            <ul>
                <li><strong>Accidental Damage:</strong> Cracks, liquid damage, and other physical damage from drops or spills.</li>
                <li><strong>Mechanical Failure:</strong> Non-covered manufacturer defects after warranty expiration.</li>
                <li><strong>Theft & Loss:</strong> Replacement of lost or stolen devices subject to claim verification.</li>
            </ul>

            <h2>3. Deductibles</h2>
            <p>A service deductible applies to each approved claim, based on the device tier. Deductibles range from $29 to $199 depending on device model and protection tier selected.</p>

            <h2>4. Claims Process</h2>
            <p>Claims may be filed through the Aether Wireless portal, mobile app, or by calling customer support. An approved claim results in a replacement device shipped within 1-3 business days.</p>

            <h2>5. Exclusions</h2>
            <p>Coverage does not apply to intentional damage, pre-existing conditions, cosmetic issues without functional impact, or devices not enrolled in the program at the time of the incident.</p>
        """
    },
    {
        "filename": "device_trade_in_process.pdf",
        "doc_id": "POL-AETH-TRADE-001",
        "title": "Device Trade-In Process",
        "body": """
            <h2>1. Eligibility Requirements</h2>
            <p>Trade-in devices must be fully functional, free of significant damage, and free of any outstanding financial obligations under a device payment plan.</p>

            <h2>2. Trade-In Valuation</h2>
            <p>Trade-in values are determined based on device model, age, condition, and market value at the time of the trade-in quote. Quotes are valid for 30 days.</p>

            <h2>3. Trade-In Steps</h2>
            <ol>
                <li><strong>Request Quote:</strong> Obtain a trade-in valuation through the online portal or in-store.</li>
                <li><strong>Backup & Reset:</strong> Back up your data and perform a factory reset before shipping.</li>
                <li><strong>Ship Device:</strong> Package the device using the provided prepaid shipping label.</li>
                <li><strong>Receive Credit:</strong> Credit is applied to your account within 5 business days of device inspection.</li>
            </ol>

            <h2>4. Inspection & Adjustment</h2>
            <p>Received devices are inspected against the quoted condition. If discrepancies are found, the trade-in value may be adjusted and the subscriber notified.</p>

            <h2>5. Return of Ineligible Devices</h2>
            <p>Devices that do not meet trade-in eligibility are returned to the subscriber at no cost if the device holds data value, or securely recycled otherwise.</p>
        """
    },
    {
        "filename": "device_upgrade_guidelines.pdf",
        "doc_id": "POL-AETH-UPG-001",
        "title": "Device Upgrade Guidelines",
        "body": """
            <h2>1. Upgrade Eligibility</h2>
            <p>Subscribers are eligible for a discounted device upgrade once their device payment plan balance reaches 50% paid or after 24 months of an active eligible plan. Eligibility is verified at the time of purchase.</p>

            <h2>2. Early Upgrade</h2>
            <p>Early upgrades may be available at full retail price with the remaining device balance folded into a new installment agreement, or via eligible trade-in credit.</p>

            <h2>3. Upgrade Process</h2>
            <ol>
                <li>Confirm eligibility through the Aether Wireless portal or in-store.</li>
                <li>Select a new device and review installment terms.</li>
                <li>Complete the trade-in (if applicable) to offset the upgrade cost.</li>
                <li>Receive and activate the new device on the existing line.</li>
            </ol>

            <h2>4. Fees & Taxes</h2>
            <p>Upgrade activation fees of $35 may apply and are disclosed prior to purchase. Applicable taxes are calculated at checkout.</p>
        """
    },
    {
        "filename": "new_line_activation_process.pdf",
        "doc_id": "POL-AETH-ACT-001",
        "title": "New Line Activation Process",
        "body": """
            <h2>1. Activation Overview</h2>
            <p>New line activations are available online, in-store, or via phone. The subscriber must select a compatible device and approved plan for the new line.</p>

            <h2>2. Required Information</h2>
            <ul>
                <li><strong>Identification:</strong> Valid government-issued photo ID for the account holder.</li>
                <li><strong>Credit Check:</strong> A soft credit check may be required for postpaid lines.</li>
                <li><strong>Compatibility:</strong> Device must support Aether Wireless network bands (5G/6G).</li>
            </ul>

            <h2>3. Activation Fees</h2>
            <p>A one-time activation fee of $35 applies per new line. Fees are waived during select promotional periods.</p>

            <h2>4. Activation Timeline</h2>
            <p>eSIM activations typically complete within minutes. Physical SIM activations complete upon delivery and SIM registration. Number porting is handled separately per the Port-In Policy.</p>
        """
    },
    {
        "filename": "payment_eligibility_terms.pdf",
        "doc_id": "POL-AETH-PAY-001",
        "title": "Payment Eligibility Terms",
        "body": """
            <h2>1. Payment Methods</h2>
            <p>Aether Wireless accepts major credit cards, debit cards, bank transfers, and ACH auto-pay. PayAsYouGo prepaid vouchers are also redeemable online.</p>

            <h2>2. Auto-Pay Discounts</h2>
            <p>Subscribers enrolled in auto-pay with a valid bank account or credit card may qualify for a monthly discount of up to $10 per line, depending on the plan tier.</p>

            <h2>3. Eligibility Requirements</h2>
            <ul>
                <li>Account must be in good standing with no suspended services.</li>
                <li>No outstanding balances from prior billing cycles.</li>
                <li>Payment method must be verified and valid.</li>
            </ul>

            <h2>4. Payment Terms</h2>
            <p>Postpaid accounts are billed in arrears on a monthly cycle. Payment is due 21 days after statement issuance as defined in the Billing & Invoice Policy.</p>

            <h2>5. Failed Payments</h2>
            <p>Failed payment attempts may incur a $5.00 returned payment fee. Repeated failures may result in service restriction pending account settlement.</p>
        """
    },
    {
        "filename": "plan_upgrade_policy.pdf",
        "doc_id": "POL-AETH-PLAN-001",
        "title": "Plan Upgrade Policy",
        "body": """
            <h2>1. Eligibility</h2>
            <p>Subscribers may upgrade their plan at any time provided the account is in good standing. Plan upgrades are effective immediately or at the start of the next billing cycle, per subscriber preference.</p>

            <h2>2. Pro-Rated Adjustments</h2>
            <p>Mid-cycle upgrades are pro-rated, with the subscriber charged the difference between the old and new plan rates for the remainder of the current billing cycle.</p>

            <h2>3. Data & Feature Availability</h2>
            <p>Upgraded plans unlock additional data allowances, roaming, and features (e.g., tethering, premium QoS) immediately upon activation.</p>

            <h2>4. Downgrade Restrictions</h2>
            <p>Downgrades take effect at the start of the next billing cycle and may require forfeiture of promotional benefits associated with the current plan.</p>

            <h2>5. Promotional Lock-In</h2>
            <p>Plans acquired under a promotional rate may have a lock-in period. Upgrading away from such plans may forfeit remaining promotional benefits.</p>
        """
    },
    {
        "filename": "port_in_policy.pdf",
        "doc_id": "POL-AETH-PORT-001",
        "title": "Port-In Policy",
        "body": """
            <h2>1. Overview</h2>
            <p>Number porting allows subscribers to transfer their existing phone number from another carrier to Aether Wireless while retaining the same number.</p>

            <h2>2. Eligibility Requirements</h2>
            <ul>
                <li>The number must be active and in good standing with the current carrier.</li>
                <li>The subscriber must provide the correct account number and PIN/transfer PIN.</li>
                <li>No pending port-block requests on the number.</li>
            </ul>

            <h2>3. Porting Timeline</h2>
            <p>Ports typically complete within 1-3 business days. Wireless-to-wireless ports may complete within hours, while landline ports may take up to 7 days.</p>

            <h2>4. Service Coverage During Port</h2>
            <p>A temporary number is assigned until the port completes. Service on the temporary number is seamless, and the ported number replaces it upon completion.</p>

            <h2>5. Failed Ports</h2>
            <p>If a port fails due to incorrect information, the subscriber is notified and may resubmit with corrected details. Repeated failures may require contact with the releasing carrier.</p>
        """
    },
    {
        "filename": "postpaid_plan_guidelines.pdf",
        "doc_id": "POL-AETH-POST-001",
        "title": "Postpaid Plan Guidelines",
        "body": """
            <h2>1. Overview</h2>
            <p>Postpaid plans provide monthly service billed in arrears after usage. They offer the flexibility of unlimited data tiers, device financing, and premium features.</p>

            <h2>2. Credit Requirements</h2>
            <p>Postpaid enrollments require a credit assessment. Approval and deposit requirements are determined based on the applicant's credit profile.</p>

            <h2>3. Billing</h2>
            <p>Postpaid accounts are billed on a monthly cycle. Payment is due 21 days from the statement date per the Billing & Invoice Policy.</p>

            <h2>4. Device Financing</h2>
            <p>Eligible subscribers may finance devices over 12, 24, or 36 months at 0% APR subject to credit approval and plan qualification.</p>

            <h2>5. Service Suspension</h2>
            <p>Accounts past due by more than 30 days may be subject to service suspension. Restoration requires full payment of the outstanding balance plus any applicable reconnection fee.</p>
        """
    },
    {
        "filename": "promotions_eligibility_terms.pdf",
        "doc_id": "POL-AETH-PROMO-001",
        "title": "Promotions Eligibility Terms",
        "body": """
            <h2>1. General Eligibility</h2>
            <p>Promotional offers are available to new and existing subscribers subject to meeting the specific eligibility criteria of each promotion. Offers may not be combined unless expressly stated.</p>

            <h2>2. New Customer Offers</h2>
            <p>New customer promotions require a new line activation and, where applicable, a qualifying trade-in device. Offer details, duration, and credits are disclosed at the point of sale.</p>

            <h2>3. Existing Customer Offers</h2>
            <p>Existing customer promotions may require a plan upgrade, device upgrade, or add-a-line activation. Credits are applied monthly over the promotion term.</p>

            <h2>4. Promotional Credits</h2>
            <p>Monthly promotional credits are applied to the bill and appear as line-item discounts. Credits cease if the qualifying plan or device is changed before the term ends.</p>

            <h2>5. Expiration & Terms</h2>
            <p>All promotions are subject to availability and may be withdrawn at any time. Full terms and conditions are available on the promotional offer page at the time of purchase.</p>
        """
    },
    {
        "filename": "returns_refunds_policy.pdf",
        "doc_id": "POL-AETH-RET-001",
        "title": "Returns & Refunds Policy",
        "body": """
            <h2>1. Return Window</h2>
            <p>Devices and accessories purchased from Aether Wireless may be returned within 14 days of the original purchase date for a full refund, subject to the terms below.</p>

            <h2>2. Return Conditions</h2>
            <ul>
                <li>Device must be undamaged and fully functional.</li>
                <li>Original packaging, accessories, and documentation must be included.</li>
                <li>No visible signs of wear or unauthorized modifications.</li>
            </ul>

            <h2>3. Restocking Fee</h2>
            <p>A restocking fee of up to $45 may apply to returns of devices that have been activated or show signs of use beyond initial inspection.</p>

            <h2>4. Refund Processing</h2>
            <p>Refunds are processed within 5-10 business days of receiving and inspecting the returned item. Refunds are issued to the original payment method.</p>

            <h2>5. Non-Returnable Items</h2>
            <p>Downloaded content, eSIM redemptions, and pre-loaded service credits are non-returnable and non-refundable once activated.</p>
        """
    },
]

for policy in policies_data:
    html_content = f"""<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>{policy['title']}</title>
    <style>{CSS_STYLE}</style>
</head>
<body>
    <div class="header-banner">
        <div class="company-logo">AETHER <span>WIRELESS</span></div>
        <div class="document-title">{policy['title']}</div>
    </div>
    <div class="meta-grid">
        <div class="meta-row">
            <div class="meta-cell meta-label">Document ID:</div>
            <div class="meta-cell meta-value">{policy['doc_id']}</div>
            <div class="meta-cell meta-label">Effective Date:</div>
            <div class="meta-cell meta-value">September 1, 2026</div>
        </div>
    </div>
    {policy['body']}
    <div class="signature-block">
        <div class="sig-grid">
            <div class="sig-col"><div class="sig-line"></div><div class="sig-title">Approved by: Legal & Regulatory Affairs</div></div>
            <div class="sig-col"><div class="sig-line"></div><div class="sig-title">Approved by: Chief Network Officer (CNO)</div></div>
        </div>
    </div>
</body>
</html>"""

    HTML(string=html_content).write_pdf(os.path.join(output_dir, policy['filename']))
    print(f"Generated: {policy['filename']}")

print(f"\nAll policies generated in '{output_dir}/'")
