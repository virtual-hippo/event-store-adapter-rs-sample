// 参考: https://dev.classmethod.jp/articles/cloudfront-lambda-url-with-post-put-request/

import { CloudFrontRequestEvent } from "aws-lambda";

const hashPayload = async (payload: string) => {
  const encoder = new TextEncoder().encode(payload);
  const hash = await crypto.subtle.digest("SHA-256", encoder);
  const hashArray = Array.from(new Uint8Array(hash));

  return hashArray.map((bytes) => bytes.toString(16).padStart(2, "0")).join("");
};

export const handler = async (event: CloudFrontRequestEvent) => {
  const request = event.Records[0].cf.request;

  console.info(`request: ${JSON.stringify(request, null, 2)}`);

  if (request.method !== "POST" && request.method !== "PUT") {
    return request;
  }

  if (!request.body?.data) {
    return request;
  }

  const body = request.body.data;
  const decodedBody = Buffer.from(body, "base64").toString("utf-8");

  request.headers["x-amz-content-sha256"] = [{ key: "x-amz-content-sha256", value: await hashPayload(decodedBody) }];

  console.info(`modifiedRequest: ${JSON.stringify(request, null, 2)}`);

  return request;
};
