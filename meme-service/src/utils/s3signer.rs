use s3::{creds::Credentials, error::S3Error, region::Region, Bucket};

#[derive(Clone)]
pub struct S3Signer {
    creds: Credentials,
    region: Region,
}

impl S3Signer {
    pub fn new(endpoint: String, access_key: &str, secret_key: &str) -> S3Signer {
        let creds =
            Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).unwrap(); // cause im not dumb
        let region = Region::Custom {
            region: Region::UsEast1.to_string(), //default
            endpoint,
        };
        S3Signer { creds, region }
    }

    pub async fn sign_get(
        &self,
        bucket: &str,
        path: String,
        exp_secs: u32,
    ) -> Result<String, S3Error> {
        Bucket::new(bucket, self.region.clone(), self.creds.clone())
            .unwrap()
            .presign_get(path, exp_secs, None)
            .await
    }
}
