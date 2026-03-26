# Cafe Tokenize

MVP smart contract tren Soroban cho bai toan:

**Nong dan token hoa mua vu ca phe, nha dau tu mua co phan, sau thu hoach nhan loi nhuan theo ty le so huu.**

Project nay tap trung vao phan contract backend, model hoa qua trinh goi von va phan phoi loi nhuan cho mot mua vu ca phe.

## Bai toan

Trong thuc te, nong dan thuong can von truoc mua vu de mua giong, phan bon, nhan cong va van hanh trang trai. `Cafe Tokenize` mo phong cach bien san luong du kien cua mua vu thanh cac `shares` de:

- Nong dan goi von som tu nha dau tu.
- Nha dau tu mua mot phan mua vu thay vi phai dau tu nguyen lo.
- Loi nhuan duoc chia minh bach theo ty le co phan da mua.
- Toan bo trang thai giao dich duoc luu tren Stellar Soroban.

## Flow contract

Contract hien tai duoc implement tai [`contracts/src/lib.rs`](contracts/src/lib.rs).

Luong nghiep vu chinh:

1. Nong dan tao mua vu bang `create_season`.
2. Nha dau tu mua co phan bang `buy_shares`.
3. Contract giu token huy dong.
4. Nong dan rut von trien khai mua vu bang `release_capital`.
5. Sau thu hoach, nong dan nap lai quy chia loi nhuan bang `settle_payout`.
6. Tung nha dau tu `claim_payout` theo so co phan dang nam giu.
7. Neu mua vu khong trien khai, admin hoac nong dan co the `cancel_season`, sau do nha dau tu `refund_investment`.

## Contract model

Contract dang luu 2 thuc the chinh:

- `HarvestSeason`: thong tin mua vu, tong so share, gia moi share, tong von da huy dong, payout pool, ngay thu hoach, trang thai.
- `InvestorPosition`: so share nha dau tu dang nam, tong so tien da dau tu, so tien da claim, trang thai refund.

Trang thai mua vu:

- `Open`: dang mo ban co phan.
- `Active`: da giai ngan von cho nong dan.
- `Settled`: nong dan da nap payout pool, nha dau tu co the claim.
- `Cancelled`: mua vu bi huy, nha dau tu duoc refund.

## Public functions

API chinh cua contract:

- `__constructor(admin)`: set dia chi admin.
- `create_season(...)`: tao mua vu moi.
- `buy_shares(season_id, investor, shares)`: mua co phan mua vu.
- `release_capital(season_id)`: giai ngan so tien da raise cho nong dan.
- `settle_payout(season_id, payout_amount)`: nong dan nap tien loi nhuan vao contract.
- `claim_payout(season_id, investor)`: nha dau tu rut phan loi nhuan cua minh.
- `cancel_season(season_id, caller)`: huy mua vu khi van con o trang thai `Open`.
- `refund_investment(season_id, investor)`: hoan tien cho nha dau tu sau khi mua vu bi huy.
- `get_season(season_id)`: doc thong tin mua vu.
- `get_position(season_id, investor)`: doc vi the dau tu cua mot dia chi.
- `admin()`: lay admin hien tai.
- `transfer_admin(current_admin, new_admin)`: chuyen quyen admin.

## Cach tinh payout

Moi nha dau tu nhan payout theo cong thuc:

```text
payout = payout_pool * investor_shares / total_shares_sold
```

Vi du:

- Mua vu ban `100` shares
- Nha dau tu A mua `40` shares
- Nha dau tu B mua `60` shares
- Nong dan nap lai `150_000` token vao `payout_pool`

Ket qua:

- A nhan `60_000`
- B nhan `90_000`

## Cau truc repo

```text
cafe-tokenize/
|-- contracts/
|   |-- Cargo.toml
|   `-- src/
|       `-- lib.rs
|-- examples/
|-- modules/
|-- scaffold/
`-- skills/
```

Phan can quan tam nhat cho project MVP nay la:

- [`contracts/Cargo.toml`](contracts/Cargo.toml)
- [`contracts/src/lib.rs`](contracts/src/lib.rs)

## Chay local

Yeu cau:

- Rust
- target `wasm32-unknown-unknown`
- Stellar / Soroban tooling neu ban muon deploy thuc te

Build va test contract:

```bash
cd contracts
cargo test
cargo build --target wasm32-unknown-unknown --release
```

## Test coverage hien co

Contract da co unit test cho cac case chinh:

- full flow tu tao mua vu den claim payout
- chan oversubscribe khi mua qua tong so share
- huy mua vu va refund nha dau tu
- transfer admin

## Diem manh cua MVP nay

- Model don gian, de demo hackathon.
- Co escrow logic bang token contract.
- Co refund flow neu mua vu khong duoc trien khai.
- Tach ro vai tro `farmer`, `investor`, `admin`.

## Gioi han hien tai

Ban nay moi la MVP, chua co:

- NFT hoac fungible token rieng cho tung mua vu
- metadata off-chain chi tiet cho nong trai, lo dat, chung nhan
- milestone theo giai doan gieo trong
- dispute resolution day du
- event logging
- fee / revenue model cho nen tang
- oracle xac minh san luong that

## Huong mo rong tiep theo

Neu muon day project thanh ban demo manh hon, nen bo sung:

1. Mint season share token cho moi mua vu thay vi chi luu `InvestorPosition`.
2. Them event cho create, buy, settle, claim, refund.
3. Bo sung metadata CID/IPFS cho hinh anh nong trai, chung nhan huu co, bao cao san luong.
4. Them co che soft cap / hard cap va deadline raise.
5. Ho tro payout nhieu dot thay vi mot lan settle duy nhat.
6. Them dashboard frontend de farmer tao campaign va investor theo doi ROI.

## Ghi chu

Day la contract giao duc va demo y tuong. Neu muon dua vao production, can review ky:

- token authorization flow
- safe math va rounding
- storage TTL / rent
- dispute / default handling
- security review va integration test voi token contract that

## License

Educational / demo use.
