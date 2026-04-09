import UIKit

@_cdecl("liquid_glass_install_overlay")
public func liquid_glass_install_overlay() {
    DispatchQueue.main.async {
        guard #available(iOS 13.0, *) else {
            return
        }

        guard
            let scene = UIApplication.shared.connectedScenes
                .first(where: { $0.activationState == .foregroundActive }) as? UIWindowScene,
            let window = scene.windows.first(where: { $0.isKeyWindow }),
            let rootView = window.rootViewController?.view
        else {
            return
        }

        let overlayTag = 0x1A551001
        if rootView.viewWithTag(overlayTag) != nil {
            return
        }

        let blur = UIBlurEffect(style: .systemUltraThinMaterial)
        let effectView = UIVisualEffectView(effect: blur)
        effectView.tag = overlayTag
        effectView.alpha = 0.94
        effectView.translatesAutoresizingMaskIntoConstraints = false
        effectView.isUserInteractionEnabled = false

        let tint = UIView()
        tint.backgroundColor = UIColor.white.withAlphaComponent(0.08)
        tint.translatesAutoresizingMaskIntoConstraints = false
        tint.isUserInteractionEnabled = false

        rootView.addSubview(effectView)
        effectView.contentView.addSubview(tint)

        NSLayoutConstraint.activate([
            effectView.leadingAnchor.constraint(equalTo: rootView.leadingAnchor),
            effectView.trailingAnchor.constraint(equalTo: rootView.trailingAnchor),
            effectView.topAnchor.constraint(equalTo: rootView.topAnchor),
            effectView.bottomAnchor.constraint(equalTo: rootView.bottomAnchor),
            tint.leadingAnchor.constraint(equalTo: effectView.contentView.leadingAnchor),
            tint.trailingAnchor.constraint(equalTo: effectView.contentView.trailingAnchor),
            tint.topAnchor.constraint(equalTo: effectView.contentView.topAnchor),
            tint.bottomAnchor.constraint(equalTo: effectView.contentView.bottomAnchor),
        ])
    }
}
