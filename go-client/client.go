package klient

import (
	"bufio"
	"compress/gzip"
	"context"
	"encoding/json"
	"errors"
	"io"
	"net/http"
	"strings"
	"sync"
	"time"
)

// InnerPacket wraps payload with timestamp. Crypto handled server-side.
type InnerPacket struct {
	Timestamp int64       `json:"timestamp"`
	Payload   interface{} `json:"payload"`
}

// KairoClient maintains an HTTP session with adaptive rate control.
type KairoClient struct {
	baseURL string
	apiKey  string
	client  *http.Client

	mu       sync.Mutex
	backoff  time.Duration
	lastSend time.Time
}

// Connect creates a client using the given API key and base URL.
func Connect(baseURL, apiKey string) *KairoClient {
	transport := &http.Transport{
		Proxy: http.ProxyFromEnvironment,
	}
	return &KairoClient{
		baseURL: strings.TrimRight(baseURL, "/"),
		apiKey:  apiKey,
		client:  &http.Client{Transport: transport},
		backoff: 0,
	}
}

// Send POSTs data to the given endpoint and decodes JSON response into out.
func (c *KairoClient) Send(ctx context.Context, endpoint string, data interface{}, out interface{}) error {
	c.mu.Lock()
	wait := c.backoff - time.Since(c.lastSend)
	if wait > 0 {
		c.mu.Unlock()
		select {
		case <-time.After(wait):
		case <-ctx.Done():
			return ctx.Err()
		}
		c.mu.Lock()
	}
	c.lastSend = time.Now()
	c.mu.Unlock()

	packet := InnerPacket{Timestamp: time.Now().Unix(), Payload: data}
	raw, err := json.Marshal(packet)
	if err != nil {
		return err
	}

	var buf strings.Builder
	gz := gzip.NewWriter(&buf)
	if _, err := gz.Write(raw); err != nil {
		return err
	}
	if err := gz.Close(); err != nil {
		return err
	}

	req, err := http.NewRequestWithContext(ctx, "POST", c.baseURL+endpoint, strings.NewReader(buf.String()))
	if err != nil {
		return err
	}
	req.Header.Set("Authorization", "Bearer "+c.apiKey)
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Content-Encoding", "gzip")

	resp, err := c.client.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusTooManyRequests {
		c.mu.Lock()
		if c.backoff == 0 {
			c.backoff = 500 * time.Millisecond
		} else if c.backoff < 2*time.Second {
			c.backoff *= 2
		}
		c.mu.Unlock()
		return errors.New("rate limited")
	}

	c.mu.Lock()
	c.backoff = c.backoff / 2
	c.mu.Unlock()

	var reader io.Reader = resp.Body
	if resp.Header.Get("Content-Encoding") == "gzip" {
		gzr, err := gzip.NewReader(resp.Body)
		if err != nil {
			return err
		}
		defer gzr.Close()
		reader = gzr
	}

	return json.NewDecoder(reader).Decode(out)
}

// Stream establishes a GET streaming connection and returns decoded messages.
func (c *KairoClient) Stream(ctx context.Context, endpoint string) (<-chan json.RawMessage, error) {
	req, err := http.NewRequestWithContext(ctx, "GET", c.baseURL+endpoint, nil)
	if err != nil {
		return nil, err
	}
	req.Header.Set("Authorization", "Bearer "+c.apiKey)

	resp, err := c.client.Do(req)
	if err != nil {
		return nil, err
	}

	ch := make(chan json.RawMessage)
	go func() {
		defer resp.Body.Close()
		defer close(ch)
		var reader io.Reader = resp.Body
		if resp.Header.Get("Content-Encoding") == "gzip" {
			gzr, err := gzip.NewReader(resp.Body)
			if err != nil {
				return
			}
			defer gzr.Close()
			reader = gzr
		}
		scanner := bufio.NewScanner(reader)
		for scanner.Scan() {
			select {
			case <-ctx.Done():
				return
			default:
			}
			if len(scanner.Bytes()) == 0 {
				continue
			}
			var msg json.RawMessage = make([]byte, len(scanner.Bytes()))
			copy(msg, scanner.Bytes())
			ch <- msg
		}
	}()
	return ch, nil
}
