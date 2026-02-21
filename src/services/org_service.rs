use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::organization::{OrgMember, Organization, OrganizationResponse};
use crate::utils::slugify::create_slug;

pub async fn create_org(
    pool: &PgPool,
    name: &str,
    user_id: Uuid,
) -> Result<Organization, AppError> {
    let slug = create_slug(name);

    let org: Organization =
        sqlx::query_as("INSERT INTO organizations (name, slug) VALUES ($1, $2) RETURNING *")
            .bind(name)
            .bind(&slug)
            .fetch_one(pool)
            .await
            .map_err(|e| match &e {
                sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
                    // Slug conflict — append a short suffix
                    AppError::BadRequest(
                        "An organization with this name already exists".to_string(),
                    )
                }
                _ => AppError::DatabaseError(e),
            })?;

    // Add creator as admin
    sqlx::query("INSERT INTO org_members (org_id, user_id, role) VALUES ($1, $2, 'admin')")
        .bind(org.id)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(org)
}

pub async fn get_user_orgs(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<OrganizationResponse>, AppError> {
    let orgs: Vec<OrganizationResponse> = sqlx::query_as(
        r#"
        SELECT o.id, o.name, o.slug, o.logo_url,
               COALESCE(m.role, 'member') as role
        FROM organizations o
        JOIN org_members m ON m.org_id = o.id
        WHERE m.user_id = $1
        ORDER BY o.created_at ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(orgs)
}

pub async fn get_member(pool: &PgPool, org_id: Uuid, user_id: Uuid) -> Result<OrgMember, AppError> {
    sqlx::query_as("SELECT * FROM org_members WHERE org_id = $1 AND user_id = $2")
        .bind(org_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Not a member of this organization".to_string()))
}

pub async fn is_org_admin(pool: &PgPool, org_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
    let member = get_member(pool, org_id, user_id).await?;
    Ok(member.role.as_deref() == Some("admin"))
}

pub async fn require_org_admin(pool: &PgPool, org_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    if !is_org_admin(pool, org_id, user_id).await? {
        return Err(AppError::Unauthorized("Admin access required".to_string()));
    }
    Ok(())
}
