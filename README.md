# Tên dự án

**Cafe Tokenize - Farming Investment on Blockchain**

---

# Description

Cafe Tokenize là một MVP (Minimum Viable Product) smart contract trên Soroban giải quyết bài toán tài chính nông nghiệp hiện đại:

**Mô tả**: Nông dân tokenize vụ cà phê phía trước để gọi vốn, nhà đầu tư mua cổ phần vụ cà phê, sau khi thu hoạch nông dân chia lợi nhuận theo tỷ lệ cổ phần mà nhà đầu tư nắm giữ.

**Mục đích dự án**:
- Giải quyết vấn đề thiếu vốn của nông dân trước mùa vụ
- Cung cấp cơ hội đầu tư có lợi nhuận cho nhà đầu tư
- Tạo mô hình tài chính minh bạch và an toàn trên blockchain
- Loại bỏ trung gian, tăng lợi nhuận cho cả hai bên

**Tại sao idea này**:
- Nông dân thường thiếu vốn để triển khai vụ (mua giống, phân bón, nhân công)
- Hiện tại không có kênh gọi vốn trực tiếp và đáng tin cậy
- Tokenization cho phép chia nhỏ vụ cà phê thành cổ phần dễ giao dịch
- Blockchain đảm bảo quyền sở hữu và phân phối lợi nhuận công bằng

---

# Tính năng

**1. `create_season`** - Nông dân tạo mùa vụ mới
   - Nông dân tạo một vụ cà phê mới với tổng số cổ phần và giá mỗi cổ phần
   - Contract lưu trữ thông tin vụ, trạng thái khởi tạo là `Open`
   - Nông dân có thể liệt kê nhu cầu vốn và mục tiêu gọi vốn

**2. `buy_shares`** - Nhà đầu tư mua cổ phần vụ
   - Nhà đầu tư gọi hàm này để mua cổ phần của một vụ cà phê
   - Contract nhận token từ nhà đầu tư và lưu lại số cổ phần họ nắm giữ
   - Tự động cập nhật trạng thái vụ từ `Open` → `Active` khi đủ vốn gọi được

**3. `release_capital`** - Giải ngân vốn cho nông dân
   - Nông dân hoặc admin gọi để rút vốn từ contract
   - Contract chuyển toàn bộ token đã gọi được cho nông dân
   - Vụ chuyển sang trạng thái `Active` - đang triển khai

**4. `settle_payout`** - Nông dân nộp lợi nhuận sau thu hoạch
   - Nông dân nộp token lợi nhuận vào contract
   - Contract tính toán phần nhuận của mỗi nhà đầu tư theo tỷ lệ cổ phần
   - Vụ chuyển sang trạng thái `Settled` - sẵn sàng chia lợi nhuận

**5. `claim_payout`** - Nhà đầu tư nhận lợi nhuận
   - Nhà đầu tư rút phần lợi nhuận của họ theo công thức: 
   ```
   payout = payout_pool × (investor_shares / total_shares_sold)
   ```

**6. `cancel_season`** - Hủy vụ (nếu không triển khai)
   - Admin hoặc nông dân có thể hủy vụ khi đang trong trạng thái `Open`
   - Vụ chuyển sang trạng thái `Cancelled`

**7. `refund_investment`** - Hoàn tiền cho nhà đầu tư
   - Khi vụ bị hủy, nhà đầu tư nhận lại toàn bộ khoản đầu tư
   - Contract chuyển token hoàn lại từ escrow này sang ví nhà đầu tư

**8. Query functions**
   - `get_season(season_id)`: Xem thông tin chi tiết vụ cà phê
   - `get_position(season_id, investor)`: Xem vị thế đầu tư của một địa chỉ
   - `admin()`: Lấy địa chỉ admin hiện tại

---

# Contract

**Contract Address (Testnet):**
```
CDBQUOB6W2ROJLESW6UUB2M4MZQZ2HKWCWGGRXQREMO3HAVKLMUS6GZW
```

**Xem trên Stellar Expert:**
[https://stellar.expert/explorer/testnet/contract/CDBQUOB6W2ROJLESW6UUB2M4MZQZ2HKWCWGGRXQREMO3HAVKLMUS6GZW](https://stellar.expert/explorer/testnet/contract/CDBQUOB6W2ROJLESW6UUB2M4MZQZ2HKWCWGGRXQREMO3HAVKLMUS6GZW)

**Ảnh chụp màn hình Contract:**  
<img width="1910" height="1027" alt="image" src="https://github.com/user-attachments/assets/1834ac5b-dd4f-47e2-b51d-b09bddef7e54" />

---

# Future scopes

1. **Mở rộng tính năng cơ bản**
   - Hỗ trợ nhiều loại cây trồng (cà phê, cacao, lúa, ngô, v.v.)
   - Thêm NFT certificate cho mỗi vụ để xác minh quyền sở hữu
   - Tạo marketplace giao dịch cổ phần vụ

2. **Payout nâng cao**
   - Hỗ trợ thanh toán lợi nhuận nhiều lần thay vì một lần duy nhất (milestone-based)
   - Thêm cơ chế giữ lại một phần lợi nhuận cho phát triển bền vững
   - Tính toán lợi nhuận tự động dựa trên giá cà phê hiện tại (oracle)

3. **Metadata & Transparency**
   - Lưu trữ thông tin chi tiết: ảnh nông trại, chứng chỉ hữu cơ, báo cáo sản lượng
   - Tích hợp IPFS/Arweave để lưu metadata off-chain
   - Dashboard tracking tiến độ vụ mạnh mẽ

4. **Quản lý nhập lâu dài**
   - Admin dashboard để quản lý nhiều vụ
   - Tạo template vụ cà phê lặp lại hàng năm
   - Tích hợp payment gateway để nhà đầu tư dễ nạp tiền

5. **Vấn đề tranh chấp & Bảo đảm**
   - Thêm logic giải quyết tranh chấp nếu sản lượng thực tế thấp hơn dự báo
   - Quy trình kiểm định độc lập để xác minh sản lượng
   - Bảo hiểm nông nghiệp tích hợp

6. **Production & Mainnet deployment**
   - Security audit & testing hoàn chỉnh
   - Triển khai lên Mainnet Stellar
   - Tích hợp với các sàn giao dịch (DEX) lớn

---

# Profile

**Nickname / Tên:** nguyen minh nhat

**Kỹ năng:**
- Rust & Soroban Smart Contract Development
- Blockchain & Stellar Protocol
- DeFi & Tokenization Models
- Full-stack development (Frontend + Backend)
- Agricultural/Supply Chain Domain

---

## Cấu trúc Repository

```
cafe-tokenize/
├── contracts/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs          (Toàn bộ contract logic)
├── examples/
├── modules/
├── scaffold/
└── skills/
```

## Các thực thể chính (Entities)

**HarvestSeason:**
- `id`: ID duy nhất của vụ
- `farmer`: Địa chỉ nông dân tạo vụ
- `total_shares`: Tổng số cổ phần của vụ
- `share_price`: Giá mỗi cổ phần (tính bằng token)
- `min_target`: Mục tiêu vốn tối thiểu
- `max_target`: Mục tiêu vốn tối đa
- `shares_sold`: Số cổ phần đã bán
- `capital_raised`: Tổng vốn gọi được
- `payout_pool`: Tổng lợi nhuận để chia
- `harvest_date`: Ngày dự kiến thu hoạch
- `status`: Trạng thái (`Open`, `Active`, `Settled`, `Cancelled`)

**InvestorPosition:**
- `investor`: Địa chỉ nhà đầu tư
- `season_id`: ID vụ mà nhà đầu tư tham gia
- `shares_amount`: Số cổ phần nhà đầu tư nắm giữ
- `investment_amount`: Tổng tiền nhà đầu tư đã đầu tư
- `claimed_payout`: Lợi nhuận đã rút
- `is_refunded`: Có được hoàn tiền hay không

## Trạng thái Vụ (Season States)

- **Open**: Đang mở bán cổ phần
- **Active**: Đã giải ngân vốn, nông dân đang triển khai vụ
- **Settled**: Nông dân đã nộp lợi nhuận, nhà đầu tư có thể nhận
- **Cancelled**: Vụ bị hủy, nhà đầu tư được hoàn tiền

## Build & Test

### Chạy tests:

```bash
cd contracts
cargo test
```

### Build sang WASM:

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Deploy lên Testnet:

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/cafe_tokenize.wasm \
  --source-account <identity> \
  --network testnet
```

## Test Coverage hiện có

Contract đã có unit test cho các case chính:
- ✅ Full flow từ tạo vụ → claim payout
- ✅ Chặn over-subscribe khi mua quá tổng số share
- ✅ Hủy vụ và refund nhà đầu tư
- ✅ Transfer admin
- ✅ Tính toán payout chính xác

## Ghi chú

Đây là contract giáo dục và demo ý tưởng. Nếu muốn đưa vào production, cần review kỹ lưỡng các điểm:

- Token authorization flow & escrow safety
- Safe math & rounding trong tính toán payout
- Storage TTL & rent trên Soroban
- Dispute & force majeure handling
- Security audit từ chuyên gia
- Integration test với token contract thật

---

**License:** Educational / Demo use
