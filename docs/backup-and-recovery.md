# Backup and Recovery

## 1. Purpose

This document explains how bio-com protects its data and recovers from failures.

It covers:

- Database backups.
- Backup frequency.
- Backup retention.
- Point-in-time recovery.
- Data restoration.
- Disaster recovery.
- Recovery testing.
- RPO and RTO.

The goal is simple: if something goes wrong, we must be able to restore the platform and its business data.

## 2. Recovery Principles

The backup strategy follows these principles:

### Protect Business Data

PostgreSQL contains important business information, including:

- Customer accounts.
- Products and categories.
- Shopping carts.
- Orders.
- Payments.
- Refunds.
- Inventory.
- Audit records.

Losing this data can directly affect customers and business operations.

### Automate Backups

Backups should run automatically. They should not depend on someone remembering to execute a command.

### Keep Backups Separate

A backup must not exist only on the same server as the production database.

If the server fails, both the database and its local backup could be lost.

### Test Restoration

A successful backup command does not prove that the backup is usable.

Restoration must be tested regularly.

### Keep Recovery Simple

Start with PostgreSQL backup capabilities. Introduce more advanced infrastructure only when business requirements justify it.

## 3. Recovery Objectives

Two important measurements define the recovery strategy.

### Recovery Point Objective (RPO)

RPO answers:

**How much data can we afford to lose?**

For example:

If the RPO is one hour, the business accepts losing at most one hour of recent data.

### Recovery Time Objective (RTO)

RTO answers:

**How long can the platform remain unavailable?**

For example:

If the RTO is four hours, the platform should be restored within four hours of a major incident.

### Initial Targets

These are proposed starting targets, not guaranteed recovery capabilities.

| Metric | Initial Target |
|---|---|
| RPO | 1 hour |
| RTO | 4 hours |
| Full backup | Daily |
| Backup verification | Automated |
| Restoration test | Monthly |
| Backup retention | 30 days |

The final targets must be validated against the business requirements, database size, hosting environment, and available recovery infrastructure.

## 4. Backup Strategy

PostgreSQL provides several backup options.

For bio-com, use a combination of:

1. Logical backups.
2. Physical backups.
3. Write-Ahead Log (WAL) archiving when point-in-time recovery is required.

### Logical Backup

A logical backup exports database objects and data into a portable format.

PostgreSQL provides:

```bash
pg_dump
```

Example:

```bash
pg_dump \
  --format=custom \
  --no-owner \
  --file=bio-com-backup.dump \
  "$DATABASE_URL"
```

Restore with:

```bash
pg_restore \
  --no-owner \
  --dbname="$DATABASE_URL" \
  bio-com-backup.dump
```

Logical backups are useful for:

- Small and medium databases.
- Database migration.
- Moving data between environments.
- Restoring selected database objects.

They can take longer to restore as the database grows.

### Physical Backup

A physical backup copies PostgreSQL database files in a consistent manner.

PostgreSQL provides tools such as:

- `pg_basebackup`.
- WAL archiving.

Physical backups are useful for larger databases and full database recovery.

They are tied more closely to PostgreSQL versions and database cluster structure.

### Write-Ahead Log (WAL)

PostgreSQL records database changes in WAL files.

When WAL archiving is configured alongside a suitable base backup, PostgreSQL can restore a database to a specific point in time.

This capability is called Point-in-Time Recovery (PITR).

It helps recover from incidents such as:

- Accidental deletion.
- Incorrect SQL updates.
- Application bugs that modify data.
- Database corruption.

## 5. Initial Backup Plan

Start with a simple backup schedule.

| Backup | Frequency | Retention |
|---|---|---|
| Logical backup | Daily | 30 days |
| Physical base backup | According to recovery requirements | Defined by storage capacity |
| WAL archive | Continuous, when PITR is enabled | According to RPO and retention policy |

For an initial deployment, daily logical backups may be sufficient if the business accepts the potential data loss.

If the one-hour RPO is required, daily logical backups alone are not sufficient. Configure continuous WAL archiving and test PITR.

## 6. Backup Storage

Backups must be stored separately from the production database.

Recommended storage characteristics:

- Encrypted at rest.
- Restricted access.
- Separate from the production server.
- Protected against accidental deletion.
- Monitored for storage capacity.
- Covered by a retention policy.

For production, use a separate backup destination, such as an encrypted object-storage bucket.

Do not rely only on a Docker volume attached to the production host.

## 7. Backup Naming

Use a predictable naming convention.

Example:

```text
bio-com/
└── postgres/
    ├── daily/
    │   ├── bio-com-2026-11-01.dump
    │   ├── bio-com-2026-11-02.dump
    │   └── bio-com-2026-11-03.dump
    └── wal/
        └── archived-wal-files/
```

The date in each filename should represent the backup creation date.

Store metadata with each backup where practical:

- Backup timestamp.
- PostgreSQL version.
- Database identifier.
- Backup type.
- Backup status.
- Checksum.

Never store database passwords in backup filenames or metadata.

## 8. Backup Automation

Backups should run automatically.

A simple scheduled job can execute the backup command.

Example:

```bash
#!/usr/bin/env bash

set -euo pipefail

BACKUP_DIR="/backups/bio-com"
BACKUP_FILE="bio-com-$(date -u +%Y-%m-%dT%H-%M-%SZ).dump"

mkdir -p "$BACKUP_DIR"

pg_dump \
  --format=custom \
  --no-owner \
  --file="$BACKUP_DIR/$BACKUP_FILE" \
  "$DATABASE_URL"

sha256sum "$BACKUP_DIR/$BACKUP_FILE" \
  > "$BACKUP_DIR/$BACKUP_FILE.sha256"
```

The production job must also:

- Upload the backup to separate storage.
- Verify the upload.
- Report failures.
- Apply the retention policy.
- Avoid exposing credentials in logs.

Use a scheduler provided by the hosting platform or a simple scheduled job.

No additional message broker or distributed scheduler is required.

## 9. Backup Retention

Backups consume storage, so retention must be defined.

Initial policy:

- Keep daily backups for 30 days.
- Keep selected monthly backups for longer-term retention if required.
- Retain WAL files for the required PITR recovery window.
- Delete expired backups automatically only after confirming newer backups are valid.

The retention policy must also consider legal, financial, and business requirements.

Do not delete WAL files needed to restore a retained base backup.

## 10. Restoration Procedure

Restoration must be performed carefully.

A basic logical restore procedure:

### Step 1: Identify the Incident

Determine:

- What happened?
- When did it happen?
- Which data is affected?
- Is the production database still usable?

### Step 2: Select a Backup

Choose the most appropriate valid backup.

Check:

- Backup timestamp.
- Backup integrity.
- Database version compatibility.
- Available WAL files if PITR is required.

### Step 3: Prepare a Recovery Database

Do not immediately overwrite the production database.

Restore into a separate database or environment first.

### Step 4: Restore the Backup

Example:

```bash
pg_restore \
  --no-owner \
  --dbname="$RECOVERY_DATABASE_URL" \
  bio-com-backup.dump
```

For PITR, follow the PostgreSQL recovery procedure using the selected base backup and required WAL archive.

### Step 5: Validate the Data

Check:

- Database connectivity.
- Required tables.
- User records.
- Orders.
- Payments.
- Refund records.
- Inventory consistency.
- Migration version.

Run application smoke tests.

### Step 6: Switch Traffic

After validation, switch the application to the recovered database.

The change must be controlled and documented.

### Step 7: Monitor

Monitor errors, database connections, and business operations after recovery.

## 11. Point-in-Time Recovery

PITR is important when the latest backup is not recent enough.

Example:

- Daily backup completed at 01:00.
- An accidental SQL command deletes orders at 10:30.
- The incident is discovered at 11:00.

With a valid base backup and continuous WAL archiving, PostgreSQL can be restored to a point immediately before the destructive operation.

The recovery process generally involves:

1. Selecting a valid base backup.
2. Restoring the database files.
3. Configuring the WAL archive.
4. Setting the recovery target time.
5. Starting PostgreSQL in recovery mode.
6. Validating the recovered database.

PITR requires correct PostgreSQL configuration and continuous WAL archive management. It must be tested before being relied upon.

## 12. Disaster Recovery

A disaster may affect the entire production environment.

Examples:

- Server failure.
- Storage failure.
- Accidental database deletion.
- Data corruption.
- Security incident.
- Hosting provider outage.

Recovery should follow a documented sequence:

1. Declare the incident.
2. Identify the affected infrastructure.
3. Protect remaining data and evidence.
4. Provision a replacement environment.
5. Restore PostgreSQL.
6. Apply the required application version.
7. Validate business data.
8. Restore application traffic.
9. Monitor the platform.
10. Document the incident and lessons learned.

Keep application deployment artifacts and recovery instructions available independently of the production server.

## 13. Business Data Validation

A database can start successfully while containing inconsistent business data.

Recovery validation must include important business relationships.

Examples:

- Every order belongs to a valid customer.
- Order items reference valid products.
- Payment records are associated with the correct orders.
- Refund records reference valid payments.
- Inventory quantities remain valid.
- Audit records are preserved.

Do not rely only on checking whether PostgreSQL accepts connections.

## 14. Financial Data Considerations

Orders, payments, and refunds require special attention.

After recovery:

- Verify payment and refund records.
- Compare internal records with payment provider records.
- Identify operations that may have been interrupted.
- Avoid processing the same refund twice.
- Verify order and payment statuses.
- Reconcile financial records before resuming affected operations.

A restored database does not automatically reverse transactions already completed by an external payment provider.

Use idempotency and reconciliation procedures to handle these cases safely.

## 15. Security Requirements

Backup security is part of application security.

Rules:

- Encrypt backups at rest.
- Use secure transfer protocols.
- Restrict backup access.
- Keep credentials outside backup scripts.
- Rotate backup credentials.
- Record backup and restoration activity.
- Avoid downloading production backups to developer machines.
- Apply retention and deletion policies.
- Protect backups from unauthorized modification.

For sensitive incidents, preserve relevant backups and logs according to the incident response process.

## 16. Recovery Testing

Backups must be tested regularly.

Monthly restoration test:

1. Select a recent backup.
2. Create an isolated PostgreSQL instance.
3. Restore the backup.
4. Validate database structure.
5. Check important business records.
6. Run application smoke tests.
7. Measure restoration time.
8. Record the result.

Track:

| Test | Expected Result |
|---|---|
| Backup creation | Successful |
| Backup upload | Successful |
| Checksum validation | Successful |
| Database restoration | Successful |
| Business data validation | Successful |
| Application startup | Successful |
| Recovery duration | Within RTO |

A failed recovery test must create an action item.

## 17. Operational Responsibilities

| Responsibility | Owner |
|---|---|
| Backup automation | Platform operations |
| Backup storage | Platform operations |
| Backup verification | Platform operations |
| Restore testing | Development / operations |
| Recovery approval | Authorized operations owner |
| Business data validation | Development / business operations |
| Incident documentation | Incident owner |

The exact ownership can be adapted as the team grows.

## 18. Initial Technology Decisions

| Responsibility | Choice |
|---|---|
| Database | PostgreSQL |
| Logical backup | pg_dump |
| Logical restore | pg_restore |
| Physical backup | pg_basebackup |
| Transaction log | PostgreSQL WAL |
| Point-in-time recovery | PostgreSQL PITR |
| Backup storage | Separate encrypted storage |
| Scheduling | Hosting platform scheduler or scheduled job |
| Integrity verification | SHA-256 checksum |

## 19. Key Design Decisions

1. PostgreSQL is the source of truth for business data.
2. Backups run automatically.
3. Backups are stored separately from production.
4. Logical backups provide a simple initial recovery option.
5. PITR is required if the one-hour RPO is adopted.
6. Restoration must be tested regularly.
7. Production recovery must include business data validation.
8. Payment and refund records require reconciliation after recovery.
9. Recovery procedures must be documented.
10. Backup access must be restricted and audited.

---

**Main principle:** A backup is useful only when it can be restored, validated, and used to bring the business back online.
